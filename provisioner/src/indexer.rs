use std::{
    fs::OpenOptions,
    io::ErrorKind,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use log::{debug, error, info};
use peaq_client::Client;
use peaq_gen::api::peaq_did::events::{AttributeAdded, AttributeRemoved, AttributeUpdated};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, SqliteConnection};
use subxt::{
    events::{EventDetails, StaticEvent},
    PolkadotConfig,
};
use tokio::sync::Mutex;

use crate::{
    config::{self, Config},
    Device, Error, DEVICE_ATTRIBUTE_NAME,
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
            debug!("get events in {} block", current_block_index);
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
        let event = event.as_event::<AttributeAdded>()?.ok_or_else::<Error, _>(|| "".into())?;
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
        let mut conn = self.conn.lock().await;
        sqlx::query(
            r#"
                insert into devices (address, data, updated_at) values (?1, ?2, ?3) 
                on conflict(address) do update set data = ?2, updated_at = ?3
            "#,
        )
        .bind(address)
        .bind(data)
        .bind(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn query(&self) -> Result<Vec<DatabaseDevice>, Error> {
        let mut conn = self.conn.lock().await;
        // Query most fresh devices.
        let devices: Vec<DatabaseDevice> = sqlx::query_as::<_, DatabaseDevice>(
            "select * from devices order by updated_at desc limit 1",
        )
        .fetch_all(&mut *conn)
        .await?;
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
}

struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(value: serde_json::Error) -> Self {
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

#[derive(Serialize, Deserialize)]
struct DeviceResponse {
    address: String,
    device: Device,
    updated_at: u64,
}

async fn get_devices(
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let internal_devices = database.query().await?;
    let mut external_devices: Vec<DeviceResponse> = Vec::with_capacity(internal_devices.len());
    for internal_device in &internal_devices {
        let device_data: Device = serde_json::from_slice(&internal_device.data)?;
        external_devices.push(DeviceResponse {
            address: internal_device.address.clone(),
            device: device_data,
            updated_at: internal_device.updated_at as u64,
        })
    }
    Ok((StatusCode::OK, Json(external_devices)))
}
