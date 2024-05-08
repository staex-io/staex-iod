use std::collections::HashMap;

use log::Level;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) log_level: String,
    pub(crate) rpc_url: String,
    // We use signer in PEAQ SignerClient to sign transactions.
    pub(crate) signer: Signer,
    pub(crate) faucet: Faucet,
    // Device represents on-chain DID information and sync options.
    pub(crate) device: Device,
    pub(crate) rbac: RBAC,
    pub(crate) indexer: Indexer,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: Level::Debug.to_string(),
            rpc_url: "wss://mpfn1.peaq.network".to_string(),
            signer: Default::default(),
            faucet: Default::default(),
            device: Default::default(),
            rbac: Default::default(),
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
    // Force update on-chain DID.
    // It disables comparison on sync.
    pub(crate) force: bool,
    pub(crate) attributes: Attributes,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            sync: true,
            force: false,
            attributes: Attributes::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct RBAC {
    // Disable RBAC init on start.
    pub(crate) init: bool,
    pub(crate) from_block: u64,
    // Entity id as a base64 string.
    pub(crate) group_id: String,
    // Entity id as a base64 string.
    pub(crate) permission_id: String,
}

impl Default for RBAC {
    fn default() -> Self {
        Self {
            init: true,
            from_block: 2158939,
            group_id: "c/IMtbTiCQDNM5rPRV3RzNVW052oLqiWpfYMwl0oN/k=".to_string(),
            permission_id: "Hz6QvvQNX3SgBn26q/HS9etIyS74gC7622JVCBanNT0=".to_string(),
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
    pub(crate) price_pin: f64,
    pub(crate) staex_mcc_id: String,
    pub(crate) mqtt_topics: Vec<String>,
    pub(crate) additional: Option<HashMap<String, toml::Value>>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            data_type: "cctv-camera".to_string(),
            location: "40.1949288120072,44.55177253802097".to_string(),
            price_access: 42.03995,
            price_pin: 445.12222,
            staex_mcc_id: "g5zkjxhge9jqjfvjm1s539xgc7pqt1h9gm59txg1xn4xazfqqbwg".to_string(),
            mqtt_topics: vec!["measurements".to_string()],
            additional: Some(HashMap::from([
                ("microcontroller".to_string(), "stm32".into()),
                ("device_age_in_years".to_string(), 2.into()),
            ])),
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
            from_block: 1731233,
            dsn: "sqlite:staex-iod.sqlite".to_string(),
            host: "127.0.0.1".to_string(),
            port: 4698,
        }
    }
}
