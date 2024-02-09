use std::{
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
use peaq_client::Client;
use peaq_gen::api::peaq_did::events::{AttributeAdded, AttributeRemoved, AttributeUpdated};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, QueryBuilder, SqliteConnection};
use subxt::{
    events::{EventDetails, StaticEvent},
    PolkadotConfig,
};
use tokio::sync::Mutex;

use crate::{
    config::{self, Config},
    Device, Error, DEVICE_ATTRIBUTE_NAME, V1,
};

pub(crate) async fn run(cfg: Config) -> Result<(), Error> {
    let database = Database::new(&cfg.indexer).await?;

    let mut indexer = Indexer::new(&cfg.clone(), database.clone()).await?;
    tokio::spawn(async move {
        if let Err(e) = indexer.run(cfg.indexer.from_block).await {
            error!("failed to run indexer: {e}");
        }
    });

    tokio::spawn(async move {
        if let Err(e) = run_api(&cfg.indexer, database).await {
            error!("failed to run api: {e}")
        }
    });

    Ok(())
}

struct Indexer {
    peaq_client: Client,
    database: Database,
}

impl Indexer {
    async fn new(cfg: &Config, database: Database) -> Result<Self, Error> {
        let peaq_client = peaq_client::Client::new(&cfg.rpc_url).await?;
        Ok(Self {
            peaq_client,
            database,
        })
    }

    async fn run(&mut self, from_block: u64) -> Result<(), Error> {
        let mut current_block_index: u64 = from_block;
        loop {
            trace!("get events in {} block", current_block_index);
            let events = self.peaq_client.get_events_in_block(current_block_index).await?;
            let events = match events {
                Some(events) => events,
                None => {
                    debug!("indexer synced all blocks; waiting for new");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            };
            for event in events.iter() {
                let event = event?;
                self.process_event(event).await?;
            }
            // Go to the next block.
            current_block_index += 1;
        }
    }

    async fn process_event(&mut self, event: EventDetails<PolkadotConfig>) -> Result<(), Error> {
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

    async fn process_added_event(
        &mut self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeAdded>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeAdded".into())?;
        if event.2.ne(DEVICE_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("added event received for {}", address);
        self.database.save(address, event.3).await
    }

    async fn process_updated_event(
        &mut self,
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
        self.database.save(address, event.3).await
    }

    async fn process_removed_event(
        &mut self,
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
        self.database.delete(address).await
    }
}

#[derive(sqlx::FromRow)]
struct DatabaseDevice {
    address: String,
    version: String,
    data: Vec<u8>,
    updated_at: i64,
}

#[derive(Clone)]
struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    async fn new(cfg: &config::Indexer) -> Result<Self, Error> {
        // Create file if not exists to be able to open and migrate.
        let file_name = cfg.dsn.split(':').collect::<Vec<&str>>()[1];
        if let Err(e) =
            OpenOptions::new().read(true).write(true).create(true).create_new(true).open(file_name)
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

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    async fn save(&mut self, address: &str, data: Vec<u8>) -> Result<(), Error> {
        let device: Device = serde_json::from_slice(&data)?;
        let data: Vec<u8> = match &device {
            Device::V1(device) => serde_json::to_vec(&device)?,
        };
        let mut conn = self.conn.lock().await;
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
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query(&self, params: GetDevicesParams) -> Result<Vec<DatabaseDevice>, Error> {
        let mut query: QueryBuilder<sqlx::Sqlite> = {
            let mut query: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new("select * from devices");
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
                    query.push_bind(&filter.value);
                }
            }
            query.push(" order by updated_at desc");
            query.push(" limit ").push_bind(params.limit).push(" offset ").push_bind(params.offset);
            query
        };
        let query = query.build_query_as::<DatabaseDevice>();
        let mut conn = self.conn.lock().await;
        let devices = query.fetch_all(&mut *conn).await?;
        Ok(devices)
    }

    async fn delete(&mut self, address: &str) -> Result<(), Error> {
        let mut conn = self.conn.lock().await;
        sqlx::query("delete from devices where address = ?1")
            .bind(address)
            .execute(&mut *conn)
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

async fn run_api(cfg: &config::Indexer, database: Database) -> Result<(), Error> {
    let app = Router::new().route("/devices", get(get_devices)).layer(Extension(database));
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
    limit: u32,
    offset: u32,
    filters: Vec<Filter>,
}

impl Default for GetDevicesParams {
    fn default() -> Self {
        Self {
            limit: 10,
            offset: 0,
            filters: vec![],
        }
    }
}

#[derive(Deserialize)]
struct Filter {
    field: String,
    condition: String, // "=", "<", ">"
    value: String,
}

#[derive(Serialize, Deserialize)]
struct DeviceResponse {
    address: String,
    version: String,
    device: serde_json::Value,
    updated_at: u64,
}

async fn get_devices(
    QueryArray(params): QueryArray<GetDevicesParams>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    for filter in &params.filters {
        if Database::is_filter_allowed(filter).is_err() {
            return Err(format!("{} field is not supporting for filtering", filter.field).into());
        }
    }
    let internal_devices = database.query(params).await?;
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
