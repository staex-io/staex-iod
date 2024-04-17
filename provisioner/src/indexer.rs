use std::{
    cmp::Ordering,
    fs::OpenOptions,
    io::ErrorKind,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use log::{debug, error, info, trace};
use peaq_client::{
    peaq_gen::api::peaq_did::events::{AttributeAdded, AttributeRemoved, AttributeUpdated},
    Client,
};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use subxt::{
    events::{EventDetails, Events, StaticEvent},
    PolkadotConfig,
};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{
    config::{self, Config},
    did::{Device, DEVICE_ATTRIBUTE_NAME, V1},
    Error,
};

pub(crate) async fn run(cfg: Config) -> Result<(), Error> {
    let database = Arc::new(Mutex::new(Database::new(&cfg.indexer).await?));

    let indexer = Indexer::new(&cfg.clone(), database.clone()).await?;
    tokio::spawn(async move { indexer.run(cfg.indexer.from_block).await });

    tokio::spawn(async move {
        if let Err(e) = run_api(&cfg.indexer, database).await {
            error!("failed to run api: {e}")
        }
    });

    Ok(())
}

struct Indexer {
    peaq_client: Client,
    database: DatabasePointer,
}

impl Indexer {
    async fn new(cfg: &Config, database: DatabasePointer) -> Result<Self, Error> {
        let peaq_client = peaq_client::Client::new(&cfg.rpc_url).await?;
        Ok(Self {
            peaq_client,
            database,
        })
    }

    async fn run(&self, from_block: u64) {
        if let Ok(latest_block) = self.peaq_client.get_last_block().await {
            trace!("current latest block is: {}", latest_block.block.header.number);
        }
        let mut current_block_index: u64 = from_block;
        let mut workers: usize = 250;
        loop {
            let saved_current_block_index = current_block_index;
            let saved_workers = workers;
            debug!("current block to sync is {current_block_index}; workers = {workers}");
            match self.process(&mut current_block_index, workers).await {
                Ok(no_more_events) => {
                    if no_more_events {
                        current_block_index = saved_current_block_index;
                        workers /= 2;
                        if workers == 0 {
                            workers = 1;
                        }
                        trace!("indexer synced all blocks; waiting for new; current workers count is {workers}");
                        sleep(Duration::from_secs(10)).await;
                    }
                }
                Err(e) => {
                    error!("failed to process starting from {saved_current_block_index}: {e}");
                    current_block_index = saved_current_block_index;
                    workers = saved_workers;
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn process(&self, current_block_index: &mut u64, workers: usize) -> Result<bool, String> {
        let (res_s, mut res_r) =
            mpsc::channel::<Result<(u64, Option<Events<PolkadotConfig>>), String>>(1);

        for _ in 0..workers {
            *current_block_index += 1;
            let local_block_index = *current_block_index;
            let peaq_client = self.peaq_client.clone();
            let res_s_ = res_s.clone();
            tokio::spawn(async move {
                trace!("get events in {} block", local_block_index);
                let events = match peaq_client.get_events_in_block(local_block_index).await {
                    Ok(events) => events,
                    Err(e) => {
                        error!(
                            "failed to get events in {local_block_index} block from peaq: {:?}",
                            e
                        );
                        if let Err(e) =
                            res_s_.send(Err("failed to get events from peaq".to_string())).await
                        {
                            error!(
                                "failed to send err result by {local_block_index} to the channel: {e}"
                            )
                        }
                        return;
                    }
                };
                if let Err(e) = res_s_.send(Ok((local_block_index, events))).await {
                    error!("failed to send ok result by {local_block_index} to the channel: {e}");
                }
            });
        }

        let mut results: Vec<(u64, Option<Events<PolkadotConfig>>)> = Vec::with_capacity(workers);
        for _ in 0..workers {
            let res = match res_r.recv().await {
                Some(res) => res,
                None => return Err("failed to receive a result from the thread, it is none".into()),
            };
            match res {
                Ok(res) => results.push(res),
                Err(_) => {
                    return Err("failed to receive a result from the thread, error received".into());
                }
            }
        }
        results.sort_by(|x, y| -> Ordering {
            if x.0 < y.0 {
                Ordering::Less
            } else if x.0 > y.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        for res in results {
            let events = match res.1 {
                Some(events) => events,
                None => return Ok(true),
            };
            self.process_events(events).await.map_err(|e| e.to_string())?;
        }

        Ok(false)
    }

    async fn process_events(&self, events: Events<PolkadotConfig>) -> Result<(), Error> {
        for event in events.iter().flatten() {
            self.process_event(event).await?;
        }
        Ok(())
    }

    async fn process_event(&self, event: EventDetails<PolkadotConfig>) -> Result<(), Error> {
        if event.pallet_name() != AttributeAdded::PALLET {
            return Ok(());
        }
        match event.variant_name() {
            AttributeAdded::EVENT => self.process_added_event(event).await,
            AttributeUpdated::EVENT => self.process_updated_event(event).await,
            AttributeRemoved::EVENT => self.process_removed_event(event).await,
            _ => Ok(()),
        }
    }

    async fn process_added_event(&self, event: EventDetails<PolkadotConfig>) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeAdded>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeAdded".into())?;
        if event.2.ne(DEVICE_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("added event received for {}", address);
        self.database.lock().await.save(address, event.3).await
    }

    async fn process_updated_event(
        &self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeUpdated>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeUpdated".into())?;
        if event.2.ne(DEVICE_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("updated event received for {}", address);
        self.database.lock().await.save(address, event.3).await
    }

    async fn process_removed_event(
        &self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeRemoved>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeRemoved".into())?;
        if event.2.ne(DEVICE_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("remove event received for {}", address);
        self.database.lock().await.delete(address).await
    }
}

#[derive(sqlx::FromRow)]
struct DatabaseDevice {
    address: String,
    version: String,
    data: Vec<u8>,
    updated_at: i64,
}

type DatabasePointer = Arc<Mutex<Database>>;

struct Database {
    conn: SqliteConnection,
}

impl Database {
    async fn new(cfg: &config::Indexer) -> Result<Self, Error> {
        // Create file if not exists to be able to open and migrate.
        let file_name = cfg.dsn.split(':').collect::<Vec<&str>>()[1];
        if let Err(e) =
            OpenOptions::new().read(true).create(true).append(true).create_new(true).open(file_name)
        {
            match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(e.into()),
            }
        }

        let mut conn = SqliteConnection::connect(&cfg.dsn).await?;
        conn.ping().await?;

        let migrator = sqlx::migrate!("./migrations/");
        migrator.run_direct(&mut conn).await?;

        Ok(Self { conn })
    }

    async fn save(&mut self, address: &str, data: Vec<u8>) -> Result<(), Error> {
        let device: Device = serde_json::from_slice(&data)?;
        let data: Vec<u8> = match &device {
            Device::V1(device) => serde_json::to_vec(&device)?,
        };
        sqlx::query(
            r#"
                insert into devices (address, version, data, updated_at) values (?1, ?2, ?3, ?4) 
                on conflict(address) do update set data = ?3, updated_at = ?4
            "#,
        )
        .bind(address)
        .bind(device.version())
        .bind(data)
        .bind(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    async fn query(&mut self, params: GetDevicesParams) -> Result<Vec<DatabaseDevice>, Error> {
        let mut query: QueryBuilder<sqlx::Sqlite> = Self::prepare_query::<sqlx::Sqlite>(&params)?;
        trace!("sql query: {}", query.sql());
        let query = query.build_query_as::<DatabaseDevice>();
        let devices = query.fetch_all(&mut self.conn).await?;
        Ok(devices)
    }

    fn prepare_query<'a, DB: sqlx::Database>(
        params: &'a GetDevicesParams,
    ) -> Result<QueryBuilder<'a, DB>, Error>
    where
        std::string::String: sqlx::Encode<'a, DB>,
        std::string::String: sqlx::Type<DB>,
        u32: sqlx::Encode<'a, DB>,
        u32: sqlx::Type<DB>,
        f64: sqlx::Encode<'a, DB>,
        f64: sqlx::Type<DB>,
    {
        let mut query: QueryBuilder<DB> = QueryBuilder::new("select * from devices");
        if let Some(address) = &params.address {
            query.push(" where address = ");
            query.push_bind(address);
            return Ok(query);
        }
        let filters_len = params.filters.len();
        if filters_len != 0 {
            query.push(" where ");
            for (i, filter) in params.filters.iter().enumerate() {
                Self::is_filter_allowed(filter)?;
                if i != 0 {
                    query.push(" AND ");
                }
                query.push(format!(
                    "json_extract(data, '$.{}') {} ",
                    filter.field, filter.condition
                ));
                push_bind(&mut query, &filter.value);
            }
        }
        query.push(" order by updated_at desc");
        query.push(" limit ").push_bind(params.limit).push(" offset ").push_bind(params.offset);
        Ok(query)
    }

    async fn delete(&mut self, address: &str) -> Result<(), Error> {
        sqlx::query("delete from devices where address = ?1")
            .bind(address)
            .execute(&mut self.conn)
            .await?;
        Ok(())
    }

    // I didn't find a way to properly bind JSON field name and condition to sql query,
    // so it is required to manually check for allowed fields and conditions.
    // For value we don't need this check as we can bind it.
    fn is_filter_allowed(filter: &Filter) -> Result<(), Error> {
        Self::is_field_allowed(&filter.field)?;
        Self::is_condition_allowed(&filter.condition)?;
        Ok(())
    }

    fn is_field_allowed(field: &str) -> Result<(), Error> {
        if matches!(field, "data_type" | "location" | "price_access" | "price_pin") {
            return Ok(());
        }
        Err("received untrusted filter".into())
    }

    fn is_condition_allowed(field: &str) -> Result<(), Error> {
        if matches!(field, "=" | "<" | ">") {
            return Ok(());
        }
        Err("received untrusted condition".into())
    }
}

fn push_bind<'a, DB: sqlx::Database>(query: &mut QueryBuilder<'a, DB>, value: &'a Value)
where
    std::string::String: sqlx::Encode<'a, DB>,
    std::string::String: sqlx::Type<DB>,
    f64: sqlx::Encode<'a, DB>,
    f64: sqlx::Type<DB>,
{
    match value {
        Value::String(string) => query.push_bind(string),
        Value::F64(f64) => query.push_bind(f64),
    };
}

struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl<T: ToString> From<T> for ErrorResponse {
    fn from(value: T) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("internal server error: {}", self.message);
        }
        if self.message.is_empty() {
            self.status_code.into_response()
        } else {
            (self.status_code, self.message).into_response()
        }
    }
}

async fn run_api(cfg: &config::Indexer, database: DatabasePointer) -> Result<(), Error> {
    let app = Router::new()
        .route("/devices", get(get_devices))
        .layer(Extension(database))
        .fallback(fallback);
    let addr = format!("{}:{}", cfg.host, cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listen on {addr} for HTTP requests");
    axum::serve(listener, app).await?;
    Ok(())
}

struct QueryArray<T>(pub T);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for QueryArray<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned + Default,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query = match parts.uri.query() {
            Some(query) => query,
            None => return Ok(Self(T::default())),
        };
        let data = match serde_qs::from_str::<T>(query) {
            Ok(data) => data,
            Err(e) => return Err(format!("failed to decode query params: {e}").into()),
        };
        Ok(Self(data))
    }
}

#[derive(Deserialize)]
struct GetDevicesParams {
    address: Option<String>,
    #[serde(default)]
    filters: Vec<Filter>,
    #[serde(default)]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

impl Default for GetDevicesParams {
    fn default() -> Self {
        Self {
            address: None,
            filters: vec![],
            limit: 10,
            offset: 0,
        }
    }
}

#[derive(Deserialize)]
struct Filter {
    field: String,
    condition: String, // "=", "<", ">"
    value: Value,
}

enum Value {
    String(String),
    F64(f64),
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.parse::<f64>() {
            Ok(v) => Ok(Value::F64(v)),
            _ => Ok(Value::String(value)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DeviceResponse {
    address: String,
    version: String,
    device: serde_json::Value,
    updated_at: u64,
}

async fn get_devices(
    Extension(database): Extension<DatabasePointer>,
    QueryArray(params): QueryArray<GetDevicesParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    for filter in &params.filters {
        if Database::is_filter_allowed(filter).is_err() {
            return Err(format!("{} field is not supporting for filtering", filter.field).into());
        }
    }
    let internal_devices = database.lock().await.query(params).await?;
    let mut external_devices: Vec<DeviceResponse> = Vec::with_capacity(internal_devices.len());
    for internal_device in &internal_devices {
        let device: serde_json::Value = {
            match internal_device.version.as_str() {
                V1 => {
                    let device: serde_json::Value = serde_json::from_slice(&internal_device.data)?;
                    device
                }
                _ => {
                    return Err(format!(
                        "unknown version to convert internal device to external :{}",
                        internal_device.version
                    )
                    .into())
                }
            }
        };
        external_devices.push(DeviceResponse {
            address: internal_device.address.clone(),
            version: internal_device.version.clone(),
            device,
            updated_at: internal_device.updated_at as u64,
        })
    }
    Ok((StatusCode::OK, Json(external_devices)))
}

async fn fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
