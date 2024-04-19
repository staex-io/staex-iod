use std::collections::HashMap;

use log::{info, warn};
use peaq_client::SignerClient;
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

use crate::{config, Error};

// Key to store information about IoT device.
pub(crate) const DEVICE_ATTRIBUTE_NAME: &str = "staex-iod-device";
// Key to store information about client.
pub(crate) const CLIENT_INFO_ATTRIBUTE_NAME: &str = "staex-iod-client";

// SyncState represent on-chain DID state.
// We need to compare local DID information with on-chain
// and decide what to do.
enum SyncState {
    Ok,
    Outdated,
    NotCreated,
}

pub(crate) const V1: &str = "v1";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Device {
    V1(DeviceV1),
}

impl Device {
    pub(crate) fn version(&self) -> &str {
        match self {
            Device::V1(_) => V1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DeviceV1 {
    data_type: String,
    location: String,
    price_access: f64,
    price_pin: f64,
    staex_mcc_id: String,
    mqtt_topics: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional: Option<HashMap<String, toml::Value>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ClientInfo {
    pub(crate) staex_mcc_id: String,
}

pub(crate) async fn update_client_info(
    rpc_url: &str,
    phrase: &Mnemonic,
    staex_mcc_id: String,
) -> Result<(), Error> {
    let keypair = Keypair::from_phrase(phrase, None)?;
    let peaq_client = peaq_client::SignerClient::new(rpc_url, keypair).await?;
    let value = serde_json::to_vec(&ClientInfo { staex_mcc_id })?;
    peaq_client.did().add_attribute(CLIENT_INFO_ATTRIBUTE_NAME, value).await?;
    Ok(())
}

pub(crate) async fn sync_did(
    cfg: &config::Device,
    peaq_client: &SignerClient,
) -> Result<(), Error> {
    if !cfg.sync {
        return Ok(());
    }
    let last_block = peaq_client.get_last_block().await?;
    info!(
        "starting to get on-chain device information starting from {} block",
        last_block.block.header.number
    );
    let device: Option<Device> = peaq_client
        .did()
        .read_attribute::<Device>(DEVICE_ATTRIBUTE_NAME, peaq_client.address().clone())
        .await?;
    let sync_state = get_sync_state(device, cfg);
    match sync_state {
        SyncState::Ok => {
            info!("on-chain device is up to date");
            if cfg.force {
                warn!("force sync is enabled; starting to sync it");
                let value = prepare_device(cfg)?;
                peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                info!("successfully updated on-chain device");
            }
        }
        SyncState::Outdated => {
            info!("on-chain device is outdated; starting to sync it");
            let value = prepare_device(cfg)?;
            peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
            info!("successfully updated on-chain device");
        }
        SyncState::NotCreated => {
            info!("on-chain device is not created");
            let value = prepare_device(cfg)?;
            peaq_client.did().add_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
            info!("successfully created on-chain device");
        }
    }
    Ok(())
}

pub(crate) async fn self_remove(peaq_client: SignerClient) -> Result<(), Error> {
    info!("starting to do self-remove");
    Ok(peaq_client.did().remove_attribute(DEVICE_ATTRIBUTE_NAME).await?)
}

pub(crate) async fn get_client_info(
    peaq_client: &SignerClient,
    address: AccountId32,
) -> Result<ClientInfo, Error> {
    match peaq_client
        .did()
        .read_attribute::<Option<ClientInfo>>(CLIENT_INFO_ATTRIBUTE_NAME, address)
        .await?
        .ok_or("on-chain client attributes are empty")?
    {
        Some(client_info) => Ok(client_info),
        None => Err("failed to decode on-chain result with client did attributes".into()),
    }
}

fn get_sync_state(device: Option<Device>, expected: &config::Device) -> SyncState {
    if device.is_none() {
        return SyncState::NotCreated;
    }
    let device = device.unwrap();
    match device {
        Device::V1(device) => {
            if device.data_type != expected.attributes.data_type
                || device.location != expected.attributes.location
                || device.price_access != expected.attributes.price_access
                || device.price_pin != expected.attributes.price_pin
                || device.staex_mcc_id != expected.attributes.staex_mcc_id
                || device.mqtt_topics != expected.attributes.mqtt_topics
            {
                SyncState::Outdated
            } else {
                SyncState::Ok
            }
        }
    }
}

fn prepare_device(cfg: &config::Device) -> Result<Vec<u8>, Error> {
    let device = Device::V1(DeviceV1 {
        data_type: cfg.attributes.data_type.clone(),
        location: cfg.attributes.location.clone(),
        price_pin: cfg.attributes.price_pin,
        price_access: cfg.attributes.price_access,
        staex_mcc_id: cfg.attributes.staex_mcc_id.clone(),
        mqtt_topics: cfg.attributes.mqtt_topics.clone(),
        additional: cfg.attributes.additional.clone(),
    });
    let value = serde_json::to_vec(&device)?;
    Ok(value)
}
