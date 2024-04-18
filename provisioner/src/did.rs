use std::collections::HashMap;

use log::{info, warn};
use peaq_client::{peaq_gen::api::peaq_did::events::AttributeRead, SignerClient};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use subxt::{
    events::{EventDetails, StaticEvent},
    utils::AccountId32,
    PolkadotConfig,
};
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

pub(crate) enum ReadResult<T> {
    Ok(T),
    DecodeError,
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
    let read_result: Option<ReadResult<Device>> = peaq_client
        .did()
        .read_attribute::<ReadResult<Device>, _>(
            DEVICE_ATTRIBUTE_NAME,
            peaq_client.address().clone(),
            Some(read_attribute_filter),
        )
        .await?;
    let sync_state = get_sync_state(read_result, cfg);
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
    let client_info = match peaq_client
        .did()
        .read_attribute::<ReadResult<ClientInfo>, _>(
            CLIENT_INFO_ATTRIBUTE_NAME,
            address,
            Some(read_attribute_filter),
        )
        .await?
        .ok_or("on-chain client attributes are empty")?
    {
        ReadResult::Ok(client) => client,
        ReadResult::DecodeError => {
            return Err("failed to decode on-chain result with client did attributes".into())
        }
    };
    Ok(client_info)
}

fn read_attribute_filter<T>(event: EventDetails<PolkadotConfig>) -> Option<ReadResult<T>>
where
    T: DeserializeOwned,
{
    if event.variant_name() == AttributeRead::EVENT {
        if let Ok(Some(evt)) = event.as_event::<AttributeRead>() {
            match serde_json::from_slice(&evt.0.value) {
                Ok(data) => return Some(ReadResult::Ok(data)),
                Err(e) => {
                    // Looks like we have outdated format.
                    warn!("failed to decode on-chain attribute: {}", e);
                    return Some(ReadResult::DecodeError);
                }
            }
        }
    }
    None
}

fn get_sync_state(read_result: Option<ReadResult<Device>>, expected: &config::Device) -> SyncState {
    if read_result.is_none() {
        return SyncState::NotCreated;
    }
    let read_result = read_result.unwrap();
    match read_result {
        ReadResult::DecodeError => SyncState::Outdated,
        ReadResult::Ok(device) => match device {
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
        },
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
