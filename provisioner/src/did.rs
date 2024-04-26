use std::collections::HashMap;

use log::{info, warn};
use peaq_client::{new_document, Document, Service, SignerClient};
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
    let peaq_client = peaq_client::SignerClient::new(rpc_url, keypair.clone()).await?;
    let doc = new_document(
        keypair.public_key().to_account_id().to_string(),
        vec![Service {
            r#type: "staex_mcc_id".to_string(),
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
                let doc = prepare_document(peaq_client.address(), cfg);
                peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, doc).await?;
                info!("successfully updated on-chain device");
            }
        }
        SyncState::Outdated => {
            info!("on-chain device is outdated; starting to sync it");
            let doc = prepare_document(peaq_client.address(), cfg);
            peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, doc).await?;
            info!("successfully updated on-chain device");
        }
        SyncState::NotCreated => {
            info!("on-chain device is not created");
            let doc = prepare_document(peaq_client.address(), cfg);
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
        if &service.r#type == "staex_mcc_id" {
            return Ok(ClientInfo {
                staex_mcc_id: service.data,
            });
        }
    }
    Err("failed to find staex mcc id to return client info".to_string().into())
}

fn get_sync_state(doc: Option<Document>, expected: &config::Device) -> SyncState {
    let doc = match doc {
        Some(doc) => doc,
        None => return SyncState::NotCreated,
    };
    if doc.services.len() != 6 {
        return SyncState::Outdated;
    }
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
            "data_type" => data_type = data,
            "location" => location = data,
            "price_access" => {
                if let Ok(data_as_f64) = data.parse() {
                    price_access = data_as_f64;
                } else {
                    return SyncState::Outdated;
                }
            }
            "price_pin" => {
                if let Ok(data_as_f64) = data.parse() {
                    price_pin = data_as_f64;
                } else {
                    return SyncState::Outdated;
                }
            }
            "staex_mcc_id" => staex_mcc_id = data,
            "mqtt_topics" => mqtt_topics.push(data),
            typ => {
                additional.insert(typ.to_string(), data.into());
            }
        }
    }
    let device = DeviceV1 {
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
    };
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

fn prepare_document(account_id: AccountId32, cfg: &config::Device) -> Document {
    // let device = Device::V1(DeviceV1 {
    //     data_type: cfg.attributes.data_type.clone(),
    //     location: cfg.attributes.location.clone(),
    //     price_pin: cfg.attributes.price_pin,
    //     price_access: cfg.attributes.price_access,
    //     staex_mcc_id: cfg.attributes.staex_mcc_id.clone(),
    //     mqtt_topics: cfg.attributes.mqtt_topics.clone(),
    //     additional: cfg.attributes.additional.clone(),
    // });
    // let value = serde_json::to_vec(&device)?;
    // Ok(value)

    let mut services = vec![
        Service {
            r#type: "data_type".to_string(),
            data: cfg.attributes.data_type.clone(),
            ..Default::default()
        },
        Service {
            r#type: "location".to_string(),
            data: cfg.attributes.location.clone(),
            ..Default::default()
        },
        Service {
            r#type: "price_pin".to_string(),
            data: cfg.attributes.price_pin.to_string(),
            ..Default::default()
        },
        Service {
            r#type: "price_access".to_string(),
            data: cfg.attributes.price_access.to_string(),
            ..Default::default()
        },
        Service {
            r#type: "staex_mcc_id".to_string(),
            data: cfg.attributes.staex_mcc_id.clone(),
            ..Default::default()
        },
    ];
    for mqtt_topic in &cfg.attributes.mqtt_topics {
        services.push(Service {
            r#type: "mqtt_topic".to_string(),
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
    new_document(account_id.to_string(), services)
}
