use std::collections::HashMap;

use log::Level;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) log_level: String,
    pub(crate) rpc_url: String,
    pub(crate) signer: Signer,
    pub(crate) faucet: Faucet,
    pub(crate) device: Device,
    pub(crate) indexer: Indexer,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: Level::Debug.to_string(),
            rpc_url: "ws://127.0.0.1:9944".to_string(),
            signer: Default::default(),
            faucet: Default::default(),
            device: Default::default(),
            indexer: Indexer::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) enum SignerType {
    Phrase,
    SecretUri,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Signer {
    pub(crate) typ: SignerType,
    pub(crate) val: String,
}

impl Default for Signer {
    fn default() -> Self {
        Self {
            typ: SignerType::SecretUri,
            val: "//Alice".to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Device {
    pub(crate) sync: bool,
    pub(crate) attributes: Attributes,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            sync: true,
            attributes: Attributes::default(),
        }
    }
}

// All fields are required attributes for every device.
// Only "additional" is additional.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Attributes {
    pub(crate) data_type: String,
    pub(crate) location: String,
    pub(crate) price_access: f64,
    pub(crate) pin_access: f64,
    pub(crate) additional: Option<HashMap<String, toml::Value>>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            data_type: "cctv-camera".to_string(),
            location: "40.1949288120072,44.55177253802097".to_string(),
            price_access: 42.03995,
            pin_access: 445.12222,
            additional: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Faucet {
    pub(crate) signer: Signer,
    // We store amount in Planck.
    pub(crate) amount: u64,
}

impl Default for Faucet {
    fn default() -> Self {
        Self {
            signer: Signer {
                typ: SignerType::SecretUri,
                val: "//Alice".to_string(),
            },
            amount: 999993264201726756, // 0.9
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Indexer {
    pub(crate) from_block: u64,
    pub(crate) dsn: String,
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl Default for Indexer {
    fn default() -> Self {
        Self {
            from_block: 1717920,
            dsn: "sqlite:staex-iod.sqlite".to_string(),
            host: "127.0.0.1".to_string(),
            port: 4698,
        }
    }
}
