use std::collections::HashMap;

use log::{debug, info, warn};
use peaq_client::{new_did_document, Document, Service, SignerClient};
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

use crate::{config, Error};

// Key to store information about IoT device.
pub(crate) const DEVICE_ATTRIBUTE_NAME: &str = "staex-iod-device";
// Key to store information about client.
pub(crate) const CLIENT_INFO_ATTRIBUTE_NAME: &str = "staex-iod-client";

const SERVICE_VERSION: &str = "staex_iod_version";
const SERVICE_DATA_TYPE: &str = "data_type";
const SERVICE_LOCATION: &str = "location";
const SERVICE_PRICE_ACCESS: &str = "price_access";
const SERVICE_PRICE_PIN: &str = "price_pin";
const SERVICE_STAEX_MCC_ID: &str = "staex_mcc_id";
const SERVICE_MQTT_TOPIC: &str = "mqtt_topic";

// SyncState represent on-chain DID state.
// We need to compare local DID information with on-chain
// and decide what to do.
enum SyncState {
    Ok,
    Outdated,
    NotCreated,
}

pub(crate) const V1: &str = "v1";

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

#[derive(Serialize, Deserialize, Debug)]
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
    let peaq_client = peaq_client::SignerClient::new(rpc_url, keypair.clone()).await?;
    let doc = new_did_document(
        &keypair,
        vec![Service {
            r#type: SERVICE_STAEX_MCC_ID.to_string(),
            data: staex_mcc_id,
            ..Default::default()
        }],
    );
    peaq_client.did().add_attribute(CLIENT_INFO_ATTRIBUTE_NAME, doc).await?;
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
    let device: Option<Document> = peaq_client
        .did()
        .read_attribute(DEVICE_ATTRIBUTE_NAME, peaq_client.address().clone())
        .await?;
    let sync_state = get_sync_state(device, cfg);
    match sync_state {
        SyncState::Ok => {
            info!("on-chain device is up to date");
            if cfg.force {
                warn!("force sync is enabled; starting to sync it");
                let doc = prepare_document(peaq_client.keypair(), cfg);
                peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, doc).await?;
                info!("successfully updated on-chain device");
            }
        }
        SyncState::Outdated => {
            info!("on-chain device is outdated; starting to sync it");
            let doc = prepare_document(peaq_client.keypair(), cfg);
            peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, doc).await?;
            info!("successfully updated on-chain device");
        }
        SyncState::NotCreated => {
            info!("on-chain device is not created");
            let doc = prepare_document(peaq_client.keypair(), cfg);
            peaq_client.did().add_attribute(DEVICE_ATTRIBUTE_NAME, doc).await?;
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
    let doc = peaq_client
        .did()
        .read_attribute(CLIENT_INFO_ATTRIBUTE_NAME, address)
        .await?
        .ok_or("on-chain client attributes are empty")?;
    for service in doc.services {
        if service.r#type == SERVICE_STAEX_MCC_ID {
            return Ok(ClientInfo {
                staex_mcc_id: service.data,
            });
        }
    }
    Err("failed to find staex mcc id to return client info".to_string().into())
}

pub(crate) fn prepare_device(doc: Document) -> Result<Device, Error> {
    let mut version: String = String::new();
    let mut data_type: String = String::new();
    let mut location: String = String::new();
    let mut price_access: f64 = 0.;
    let mut price_pin: f64 = 0.;
    let mut staex_mcc_id: String = String::new();
    let mut mqtt_topics: Vec<String> = Vec::new();
    let mut additional: HashMap<String, toml::Value> = HashMap::new();
    for service in doc.services {
        let data = service.data;
        match service.r#type.as_str() {
            SERVICE_VERSION => version = data,
            SERVICE_DATA_TYPE => data_type = data,
            SERVICE_LOCATION => location = data,
            SERVICE_PRICE_ACCESS => {
                price_access = data.parse()?;
            }
            SERVICE_PRICE_PIN => {
                price_pin = data.parse()?;
            }
            SERVICE_STAEX_MCC_ID => staex_mcc_id = data,
            SERVICE_MQTT_TOPIC => mqtt_topics.push(data),
            typ => {
                additional.insert(typ.to_string(), data.into());
            }
        }
    }
    match version.as_str() {
        V1 => Ok(Device::V1(DeviceV1 {
            data_type,
            location,
            price_access,
            price_pin,
            staex_mcc_id,
            mqtt_topics,
            additional: if additional.is_empty() {
                None
            } else {
                Some(additional)
            },
        })),
        version => Err(format!("unknown version to convert document to device: {version}").into()),
    }
}

fn get_sync_state(doc: Option<Document>, expected: &config::Device) -> SyncState {
    let doc = match doc {
        Some(doc) => doc,
        None => return SyncState::NotCreated,
    };
    if doc.services.len() < 7 {
        return SyncState::Outdated;
    }
    let device = match prepare_device(doc) {
        Ok(device) => device,
        Err(_) => return SyncState::Outdated,
    };
    match device {
        Device::V1(device) => {
            if device.data_type != expected.attributes.data_type
                || device.location != expected.attributes.location
                || device.price_access != expected.attributes.price_access
                || device.price_pin != expected.attributes.price_pin
                || device.staex_mcc_id != expected.attributes.staex_mcc_id
                || device.mqtt_topics != expected.attributes.mqtt_topics
            {
                debug!("device is outdated: {:?}", device);
                SyncState::Outdated
            } else {
                SyncState::Ok
            }
        }
    }
}

fn prepare_document(keypair: &Keypair, cfg: &config::Device) -> Document {
    let mut services = vec![
        Service {
            r#type: SERVICE_VERSION.to_string(),
            data: V1.to_string(),
            ..Default::default()
        },
        Service {
            r#type: SERVICE_DATA_TYPE.to_string(),
            data: cfg.attributes.data_type.clone(),
            ..Default::default()
        },
        Service {
            r#type: SERVICE_LOCATION.to_string(),
            data: cfg.attributes.location.clone(),
            ..Default::default()
        },
        Service {
            r#type: SERVICE_PRICE_PIN.to_string(),
            data: cfg.attributes.price_pin.to_string(),
            ..Default::default()
        },
        Service {
            r#type: SERVICE_PRICE_ACCESS.to_string(),
            data: cfg.attributes.price_access.to_string(),
            ..Default::default()
        },
        Service {
            r#type: SERVICE_STAEX_MCC_ID.to_string(),
            data: cfg.attributes.staex_mcc_id.clone(),
            ..Default::default()
        },
    ];
    for mqtt_topic in &cfg.attributes.mqtt_topics {
        services.push(Service {
            r#type: SERVICE_MQTT_TOPIC.to_string(),
            data: mqtt_topic.clone(),
            ..Default::default()
        })
    }
    if let Some(additional) = &cfg.attributes.additional {
        for (k, v) in additional.iter() {
            services.push(Service {
                r#type: k.clone(),
                data: v.to_string(),
                ..Default::default()
            })
        }
    }
    new_did_document(keypair, services)
}
