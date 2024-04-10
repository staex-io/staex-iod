use std::time::Duration;

use base64::{prelude::BASE64_STANDARD, Engine};
use log::{debug, error, info, trace};
use peaq_client::{peaq_gen::api::peaq_rbac::events::UserAssignedToGroup, SignerClient};
use tokio::{
    sync::{mpsc, watch},
    time::{sleep, timeout},
};

use crate::{config, Error};

const PERMISSION_NAME: &str = "staex_iod_mqtt_access";
const ROLE_NAME: &str = "staex_iod_accessor";
const GROUP_NAME: &str = "staex_iod_subscribers";

pub(crate) async fn init_rbac(cfg: &config::RBAC, peaq_client: &SignerClient) -> Result<(), Error> {
    if !cfg.init {
        debug!("rbac initialization skipped because of config");
        return Ok(());
    }
    info!("starting to initialize rbac entities, wait some...");
    let rbac_client = peaq_client.rbac();
    debug!("starting to add permission");
    let permission_id = rbac_client.add_permission(PERMISSION_NAME.to_string()).await.unwrap();
    debug!("starting to add role");
    let role_id = rbac_client.add_role(ROLE_NAME.to_string()).await.unwrap();
    debug!("starting to add group");
    let group_id = rbac_client.add_group(GROUP_NAME.to_string()).await.unwrap();
    debug!("starting to assign permission to role");
    rbac_client.assign_permission_to_role(permission_id, role_id).await.unwrap();
    debug!("starting to assign role to group");
    rbac_client.assign_role_to_group(role_id, group_id).await.unwrap();
    let group_id = BASE64_STANDARD.encode(group_id);
    info!("rbac entities successfully initialize; group id is {}", group_id);
    Ok(())
}

pub(crate) async fn sync_rbac(
    cfg: config::RBAC,
    peaq_client: SignerClient,
    restart_s: mpsc::Sender<()>,
    mut stop_r: watch::Receiver<()>,
) -> Result<(), Error> {
    let mut last_processed_block = if cfg.from_block == 0 {
        let latest_block = peaq_client.get_last_block().await?;
        trace!("use latest block {} to sync rbac", latest_block.block.header.number);
        latest_block.block.header.number as u64
    } else {
        trace!("use block from config {} to sync rbac", cfg.from_block);
        cfg.from_block
    };
    let mut sleep_duration = Duration::from_millis(1); // todo: use default const
    loop {
        tokio::select! {
            _ = sleep(sleep_duration) => {
                match timeout(
                        Duration::from_secs(10),
                        fetch_rbac(
                            peaq_client.clone(),
                            last_processed_block,
                            restart_s.clone()),
                        ).await {
                    Ok(res) => {
                        if let Err(e) = res {
                            error!("failed to sync rbac: {e}");
                            sleep_duration = Duration::from_secs(10); // todo: use default const
                            continue;
                        }
                    },
                    Err(e) => {
                        error!("failed to sync rbac; 10s timeout: {e}");
                        sleep_duration = Duration::from_secs(10); // todo: use default const
                        continue;
                    }
                }
                last_processed_block += 1;
                sleep_duration = Duration::from_millis(1); // todo: why? and default const
            },
            _ = stop_r.changed() => {
                debug!("received stop signal");
                return Ok(())
            }
        }
    }
}

async fn fetch_rbac(
    peaq_client: SignerClient,
    last_processed_block: u64,
    _restart_s: mpsc::Sender<()>,
) -> Result<(), Error> {
    debug!("starting to synchronize rbac with block {last_processed_block}");
    // todo: start in block to config file
    let events = peaq_client
        .get_events_in_block(last_processed_block)
        .await?
        .ok_or("empty events returned")?;
    let user_assigned_to_group_events = events.find::<UserAssignedToGroup>();
    for event in user_assigned_to_group_events.flatten() {
        eprintln!("New user assigned to group: {:?}", event);
    }
    // todo: get events from blocks like indexer and if see new rbac event -> update config -> restart mcc
    // todo: we need to do the same with rbac revocation
    Ok(())
}
