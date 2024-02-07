use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::debug;
use peaq_client::Client;
use peaq_gen::api::peaq_did::events::{AttributeAdded, AttributeRemoved, AttributeUpdated};
use sqlx::{Connection, SqliteConnection};
use subxt::{
    events::{EventDetails, StaticEvent},
    PolkadotConfig,
};

use crate::{
    config::{self, Config},
    Error, DID_ATTRIBUTE_NAME,
};

pub(crate) async fn run(cfg: Config) -> Result<(), Error> {
    let mut indexer = Indexer::new(&cfg).await?;
    indexer.run(cfg.indexer.from_block).await
}

struct Indexer {
    peaq_client: Client,
    saver: Database,
}

impl Indexer {
    async fn new(cfg: &Config) -> Result<Self, Error> {
        let peaq_client = peaq_client::Client::new(&cfg.rpc_url).await?;
        let saver = Database::new(&cfg.indexer).await?;
        Ok(Self { peaq_client, saver })
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
        if event.2.ne(DID_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("added event received for {}", address);
        self.saver.save(address, event.3).await
    }

    async fn process_updated_event(
        &mut self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeUpdated>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeUpdated".into())?;
        if event.2.ne(DID_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("updated event received for {}", address);
        self.saver.save(address, event.3).await
    }

    async fn process_removed_event(
        &mut self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeRemoved>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeRemoved".into())?;
        if event.2.ne(DID_ATTRIBUTE_NAME.as_bytes()) {
            return Ok(());
        }
        let address = &event.1.to_string();
        debug!("remove event received for {}", address);
        self.saver.delete(address).await
    }
}

struct Database {
    conn: SqliteConnection,
}

impl Database {
    async fn new(cfg: &config::Indexer) -> Result<Self, Error> {
        let mut conn = SqliteConnection::connect(&cfg.dsn).await?;
        conn.ping().await?;

        let migrator = sqlx::migrate!("./migrations/");
        migrator.run_direct(&mut conn).await?;

        Ok(Self { conn })
    }
}

impl Database {
    async fn save(&mut self, address: &str, data: Vec<u8>) -> Result<(), Error> {
        sqlx::query(
            r#"
                insert into dids (address, data, updated_at) values (?1, ?2, ?3) 
                on conflict(address) do update set data = ?2, updated_at = ?3
            "#,
        )
        .bind(address)
        .bind(data)
        .bind(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    async fn delete(&mut self, address: &str) -> Result<(), Error> {
        sqlx::query("delete from dids where address = ?1")
            .bind(address)
            .execute(&mut self.conn)
            .await?;
        Ok(())
    }
}
