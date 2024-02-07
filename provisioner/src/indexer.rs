use std::{str::from_utf8, time::Duration};

use log::{debug, error};
use peaq_client::Client;
use peaq_gen::api::peaq_did::events::{AttributeAdded, AttributeRemoved, AttributeUpdated};
use subxt::{
    events::{EventDetails, StaticEvent},
    PolkadotConfig,
};

use crate::{config::Config, Error};

pub(crate) async fn indexer(cfg: Config) {
    tokio::spawn(async move {
        let indexer = match Indexer::new(cfg).await {
            Ok(indexer) => indexer,
            Err(e) => {
                error!("failed to init indexer: {}", e);
                return;
            }
        };
        if let Err(e) = indexer.run().await {
            error!("failed to run indexer: {}", e);
        }
    });
}

struct Indexer {
    peaq_client: Client,
}

impl Indexer {
    async fn new(cfg: Config) -> Result<Self, Error> {
        let peaq_client = peaq_client::Client::new(&cfg.rpc_url).await?;
        Ok(Self { peaq_client })
    }

    async fn run(&self) -> Result<(), Error> {
        let mut current_block_index: u64 = 1713467;
        loop {
            let events = self.peaq_client.get_events_in_block(current_block_index).await?;
            let events = match events {
                Some(events) => events,
                None => {
                    debug!("indexer synced all blocks; waiting for new");
                    tokio::time::sleep(Duration::from_secs(1)).await;
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
        let event = event.as_event::<AttributeAdded>()?.ok_or_else::<Error, _>(|| "".into())?;
        eprintln!("ADDED");
        eprintln!("Name: {:?}", from_utf8(&event.2));
        eprintln!("Value: {:?}", from_utf8(&event.3));
        Ok(())
    }

    async fn process_updated_event(
        &self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeUpdated>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeUpdated".into())?;
        eprintln!("UPDATED");
        eprintln!("Name: {:?}", from_utf8(&event.2));
        eprintln!("Value: {:?}", from_utf8(&event.3));
        Ok(())
    }

    async fn process_removed_event(
        &self,
        event: EventDetails<PolkadotConfig>,
    ) -> Result<(), Error> {
        let event = event
            .as_event::<AttributeRemoved>()?
            .ok_or_else::<Error, _>(|| "event is not AttributeRemoved".into())?;
        eprintln!("REMOVED");
        eprintln!("Name: {:?}", from_utf8(&event.2));
        Ok(())
    }
}
