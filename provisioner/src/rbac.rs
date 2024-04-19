use std::{
    io::{self, BufRead, Write},
    time::Duration,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use log::{debug, error, info, trace, warn};
use peaq_client::{
    peaq_gen::api::peaq_rbac::events::UserAssignedToGroup, SignerClient, ENTITY_LENGTH,
};
use subxt::utils::AccountId32;
use tokio::{
    sync::{mpsc, watch},
    time::{sleep, timeout},
};

use crate::{config, did, vec_to_bytes, Error};

const PERMISSION_NAME: &str = "staex_iod_mqtt_access";
const ROLE_NAME: &str = "staex_iod_accessor";
const GROUP_NAME: &str = "staex_iod_subscribers";

const STAEX_MCC_CONFIG_FILE: &str = "/etc/mcc/mcc.conf";
const STAEX_MCC_CONFIG_TRUSTED_NODES_KEY: &str = "trusted-nodes";

const SLEEP_DURATION_ON_OK: Duration = Duration::from_millis(25);
const SLEEP_DURATION_ON_ERROR: Duration = Duration::from_secs(10);

pub(crate) async fn init_rbac(cfg: &config::RBAC, peaq_client: &SignerClient) -> Result<(), Error> {
    if !cfg.init {
        debug!("rbac initialization skipped because of config");
        return Ok(());
    }
    info!("starting to initialize rbac entities, wait some minutes...");
    let rbac_client = peaq_client.rbac();
    debug!("starting to add permission");
    let permission_id = rbac_client.add_permission(PERMISSION_NAME.to_string()).await.unwrap();
    debug!("permission is created: {}", BASE64_STANDARD.encode(permission_id));
    debug!("starting to add role");
    let role_id = rbac_client.add_role(ROLE_NAME.to_string()).await.unwrap();
    debug!("starting to add group");
    let group_id = rbac_client.add_group(GROUP_NAME.to_string()).await.unwrap();
    debug!("group is created: {}", BASE64_STANDARD.encode(group_id));
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
    let group_id = vec_to_bytes(BASE64_STANDARD.decode(cfg.group_id)?)?;
    let permission_id = vec_to_bytes(BASE64_STANDARD.decode(cfg.permission_id)?)?;
    let mut last_processed_block = if cfg.from_block == 0 {
        let latest_block = peaq_client.get_last_block().await?;
        trace!("use latest block {} to sync rbac", latest_block.block.header.number);
        latest_block.block.header.number as u64
    } else {
        trace!("use block from config {} to sync rbac", cfg.from_block);
        cfg.from_block
    };
    let mut sleep_duration = SLEEP_DURATION_ON_OK;
    loop {
        tokio::select! {
            _ = sleep(sleep_duration) => {
                match timeout(
                        Duration::from_secs(60),
                        fetch_rbac(
                            peaq_client.clone(),
                            group_id,
                            permission_id,
                            last_processed_block,
                            restart_s.clone()),
                        ).await {
                    Ok(res) => {
                        if let Err(e) = res {
                            warn!("failed to sync rbac: {e}");
                            sleep_duration = SLEEP_DURATION_ON_ERROR;
                            continue;
                        }
                    },
                    Err(e) => {
                        error!("failed to sync rbac; 10s timeout: {e}");
                        sleep_duration = SLEEP_DURATION_ON_ERROR;
                        continue;
                    }
                }
                last_processed_block += 1;
                sleep_duration = SLEEP_DURATION_ON_OK;
            },
            _ = stop_r.changed() => {
                debug!("received stop signal");
                return Ok(())
            }
        }
    }
}

pub(crate) async fn grant_access(
    peaq_client: &SignerClient,
    address: AccountId32,
    group_id: &String,
) -> Result<(), Error> {
    let group_id = BASE64_STANDARD.decode(group_id)?;
    let group_id = vec_to_bytes(group_id)?;
    trace!("address array is {:?}", address.0);
    trace!("group id array is {:?}", group_id);
    Ok(peaq_client.rbac().assign_user_to_group(address.0, group_id).await?)
}

pub(crate) async fn revoke_access(
    peaq_client: &SignerClient,
    address: AccountId32,
    group_id: &String,
) -> Result<(), Error> {
    let group_id = BASE64_STANDARD.decode(group_id)?;
    let group_id = vec_to_bytes(group_id)?;
    trace!("address array is {:?}", address.0);
    trace!("group id array is {:?}", group_id);
    Ok(peaq_client.rbac().unassign_user_from_group(address.0, group_id).await?)
}

async fn fetch_rbac(
    peaq_client: SignerClient,
    group_id: [u8; ENTITY_LENGTH],
    permission_id: [u8; ENTITY_LENGTH],
    last_processed_block: u64,
    restart_s: mpsc::Sender<()>,
) -> Result<(), Error> {
    debug!("starting to synchronize rbac with block {last_processed_block}");
    let events = peaq_client
        .get_events_in_block(last_processed_block)
        .await?
        .ok_or("empty events returned")?;
    let user_assigned_to_group_events = events.find::<UserAssignedToGroup>();
    for event in user_assigned_to_group_events.flatten() {
        if event.0 == peaq_client.address() && event.2 == group_id {
            let address = AccountId32::from(event.1);
            info!("received new access for {}", address);
            process_user(&peaq_client, address, &permission_id, &restart_s).await?;
        }
    }
    Ok(())
}

async fn process_user(
    peaq_client: &SignerClient,
    address: AccountId32,
    permission_id: &[u8; ENTITY_LENGTH],
    restart_s: &mpsc::Sender<()>,
) -> Result<(), Error> {
    info!("starting to get on-chain did to get staex mcc id");
    let client_info = did::get_client_info(peaq_client, address.clone()).await?;
    debug!("client staex mcc id is {} for address {address}", client_info.staex_mcc_id);

    info!("starting to fetch on-chain user permissions to double check");
    // As we parse events from blocks to know about new device access,
    // we can be sure that event is actual state,
    // because in some blocks there can be new event about access revocation.
    // So every time we need to get actual state from on-chain.
    let user_permissions =
        peaq_client.rbac().fetch_user_permissions(peaq_client.address(), address.0).await?;
    let mut found = false;
    for permission in user_permissions {
        if permission.id == *permission_id {
            found = true;
            break;
        }
    }
    if !found {
        warn!("actual permission is not found for the address: {address}");
        return Ok(());
    }

    let updated = update_current_trusted_nodes(&client_info.staex_mcc_id)?;
    if updated {
        restart_s.send(()).await?;
    }

    Ok(())
}

fn update_current_trusted_nodes(staex_mcc_id: &str) -> Result<bool, Error> {
    let staex_mcc_config = std::fs::read_to_string(STAEX_MCC_CONFIG_FILE)?;
    let lines = staex_mcc_config.split('\n').collect::<Vec<&str>>();
    let mut target_i: Option<usize> = None;
    let mut value: &str = "";
    for (i, line) in lines.iter().enumerate() {
        let param: Vec<&str> = line.split('=').collect();
        if param.len() != 2 {
            continue;
        }
        let key = param[0].trim();
        if key == STAEX_MCC_CONFIG_TRUSTED_NODES_KEY {
            target_i = Some(i);
            value = param[1].trim();
        }
    }
    match target_i {
        Some(target_i) => {
            let mut nodes = value.split(',').collect::<Vec<&str>>();
            for node in &nodes {
                if *node == staex_mcc_id {
                    debug!("staex mcc node is already exist in the config file");
                    return Ok(false);
                }
            }
            debug!("staex mcc node is not exist in the config file");
            nodes.push(staex_mcc_id);
            replace_line_in_file(
                STAEX_MCC_CONFIG_FILE,
                target_i,
                &format!("{STAEX_MCC_CONFIG_TRUSTED_NODES_KEY} = {}", nodes.join(",")),
            )?;
            Ok(true)
        }
        None => {
            debug!("trusted nodes are not present in staex mcc config file");
            std::fs::OpenOptions::new().append(true).open(STAEX_MCC_CONFIG_FILE)?.write_all(
                format!("{STAEX_MCC_CONFIG_TRUSTED_NODES_KEY} = {staex_mcc_id}\n").as_bytes(),
            )?;
            Ok(true)
        }
    }
}

fn replace_line_in_file(path: &str, line_number: usize, new_content: &str) -> io::Result<()> {
    let temp_path = format!("{}.tmp", path);
    let file = std::fs::File::open(path)?;
    let mut temp_file =
        std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(&temp_path)?;
    for (index, line) in std::io::BufReader::new(file).lines().enumerate() {
        if index == line_number {
            writeln!(temp_file, "{}", new_content)?;
        } else {
            writeln!(temp_file, "{}", line?)?;
        }
    }
    std::fs::rename(temp_path, path)?;
    Ok(())
}
