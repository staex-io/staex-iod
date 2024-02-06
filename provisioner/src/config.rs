use std::collections::HashMap;

use log::Level;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) log_level: String,
    pub(crate) rpc_url: String,
    pub(crate) signer: Signer,
    pub(crate) faucet: Faucet,
    pub(crate) did: DID,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: Level::Debug.to_string(),
            rpc_url: "ws://127.0.0.1:9944".to_string(),
            signer: Default::default(),
            faucet: Default::default(),
            did: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) enum SignerType {
    Seed,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct DID {
    pub(crate) sync: bool,
    pub(crate) explorer: bool,
    pub(crate) attributes: Attributes,
}

impl Default for DID {
    fn default() -> Self {
        Self {
            sync: true,
            explorer: true,
            attributes: Attributes::default(),
        }
    }
}

// All fields are required attributes for every DID.
// Only "additional" is additional.
#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct Attributes {
    pub(crate) data_type: String,
    pub(crate) location: String,
    pub(crate) price_access: String,
    pub(crate) pin_access: String,
    pub(crate) additional: Option<HashMap<String, toml::Value>>,
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
            amount: 100_000,
        }
    }
}
