use std::{collections::HashMap, str::FromStr};

use clap::{Parser, Subcommand};
use config::Faucet;
use log::{error, info, warn, Level, LevelFilter};
use peaq_gen::api::peaq_did::events::AttributeRead;
use serde::{Deserialize, Serialize};
use subxt::{
    config::Header,
    events::{EventDetails, StaticEvent},
    tx::Signer,
    utils::AccountId32,
    PolkadotConfig,
};
use subxt_signer::{
    bip39::{self, Mnemonic},
    sr25519::Keypair,
    SecretUri,
};

use crate::config::Config;

mod config;
mod indexer;

pub(crate) const DEVICE_ATTRIBUTE_NAME: &str = "staex-ioa-device";

pub(crate) type Error = Box<dyn std::error::Error>;

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
    pin_access: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional: Option<HashMap<String, toml::Value>>,
}

enum ReadResult {
    Ok(Device),
    DecodeError,
}

/// Command line utility to interact with StaexIoD provisioner.
#[derive(Parser)]
#[clap(name = "provisioner")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Provisioner config file path.
    #[arg(short, long)]
    #[arg(default_value = "config.toml")]
    config: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show default config for provisioner.
    Config {},
    /// Run provisioner.
    Run {},
    /// Run indexer.
    Indexer {},
    /// Create new account.
    NewAccount {},
    /// Remove on-chain device.
    SelfRemove {},
    /// Faucet account.
    Faucet {
        /// Address to send tokens to.
        address: AccountId32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let cfg: config::Config = || -> Result<config::Config, Error> {
        let buf = match std::fs::read_to_string(cli.config) {
            Ok(buf) => buf,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(config::Config::default())
            }
            Err(e) => return Err(e.into()),
        };
        let cfg: config::Config = toml::from_str(&buf)?;
        Ok(cfg)
    }()?;
    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("provisioner", cfg.log_level.parse::<Level>()?.to_level_filter())
        .init();
    match cli.command {
        Commands::Config {} => {
            eprint!("{}", toml::to_string_pretty(&config::Config::default())?);
        }
        Commands::Run {} => {
            let app: App = App::new(cfg).await?;
            tokio::spawn(async move {
                if let Err(e) = app.run().await {
                    error!("failed to run application: {e}")
                }
            })
            .await?;
        }
        Commands::Indexer {} => {
            indexer::run(cfg).await?;
            tokio::signal::ctrl_c().await?;
        }
        Commands::SelfRemove {} => {
            let app: App = App::new(cfg).await?;
            app.self_remove().await?;
        }
        Commands::NewAccount {} => {
            let (phrase, _, account_id) = generate_account()?;
            eprintln!("Phrase: {}", phrase);
            eprintln!("Address: {}", account_id);
        }
        Commands::Faucet { address } => {
            let app: App = App::new(cfg).await?;
            app.faucet(address).await?;
        }
    };
    Ok(())
}

struct App {
    peaq_client: peaq_client::SignerClient,
    faucet: Faucet,
    device: config::Device,
}

impl App {
    async fn new(cfg: Config) -> Result<Self, Error> {
        let keypair = get_keypair(&cfg.signer)?;
        let peaq_client = peaq_client::SignerClient::new(&cfg.rpc_url, keypair).await?;
        Ok(Self {
            peaq_client,
            faucet: cfg.faucet,
            device: cfg.device,
        })
    }

    async fn run(&self) -> Result<(), Error> {
        self.sync().await
    }

    async fn sync(&self) -> Result<(), Error> {
        if !self.device.sync {
            return Ok(());
        }
        let last_block = self.peaq_client.get_last_block().await?;
        info!(
            "starting to get on-chain device information starting from {} block",
            last_block.block.header.number()
        );
        let read_result: Option<ReadResult> = self
            .peaq_client
            .read_attribute::<ReadResult, _>(DEVICE_ATTRIBUTE_NAME, Some(filter))
            .await?;
        let sync_state = get_sync_state(read_result, &self.device);
        match sync_state {
            SyncState::Ok => {
                info!("on-chain device is up to date");
                if self.device.force {
                    warn!("force sync is enabled; starting to sync it");
                    let value = self.prepare_device()?;
                    self.peaq_client.update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                    info!("successfully updated on-chain device");
                }
            }
            SyncState::Outdated => {
                info!("on-chain device is outdated; starting to sync it");
                let value = self.prepare_device()?;
                self.peaq_client.update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                info!("successfully updated on-chain device");
            }
            SyncState::NotCreated => {
                info!("on-chain device is not created");
                let value = self.prepare_device()?;
                self.peaq_client.add_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                info!("successfully created on-chain device");
            }
        }
        Ok(())
    }

    async fn faucet(&self, account_id: AccountId32) -> Result<(), Error> {
        let signer = get_keypair(&self.faucet.signer)?;
        let faucet_account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&signer);
        let balance = self.peaq_client.get_balance(&faucet_account_id).await?;
        info!("faucet balance: {}: {}", faucet_account_id, balance,);

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance before: {}", balance);

        self.peaq_client.transfer(self.faucet.amount as u128, account_id.clone(), &signer).await?;

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance after: {}", balance);
        Ok(())
    }

    async fn self_remove(&self) -> Result<(), Error> {
        info!("starting to do self-remove");
        self.peaq_client.remove_attribute(DEVICE_ATTRIBUTE_NAME).await
    }

    fn prepare_device(&self) -> Result<Vec<u8>, Error> {
        let device = Device::V1(DeviceV1 {
            data_type: self.device.attributes.data_type.clone(),
            location: self.device.attributes.location.clone(),
            pin_access: self.device.attributes.pin_access,
            price_access: self.device.attributes.price_access,
            additional: self.device.attributes.additional.clone(),
        });
        let value = serde_json::to_vec(&device)?;
        Ok(value)
    }
}

pub(crate) fn generate_account() -> Result<(Mnemonic, Keypair, AccountId32), Error> {
    let phrase = bip39::Mnemonic::generate(12)?;
    let keypair = Keypair::from_phrase(&phrase, None)?;
    let account_id: AccountId32 =
        <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
    Ok((phrase, keypair, account_id))
}

fn get_keypair(cfg: &config::Signer) -> Result<Keypair, Error> {
    match cfg.typ {
        config::SignerType::Phrase => {
            Ok(Keypair::from_phrase(&bip39::Mnemonic::from_str(&cfg.val)?, None)?)
        }
        config::SignerType::SecretUri => Ok(Keypair::from_uri(&SecretUri::from_str(&cfg.val)?)?),
    }
}

fn filter(event: EventDetails<PolkadotConfig>) -> Option<ReadResult> {
    if event.variant_name() == AttributeRead::EVENT {
        if let Ok(Some(evt)) = event.as_event::<AttributeRead>() {
            match serde_json::from_slice(&evt.0.value) {
                Ok(device) => return Some(ReadResult::Ok(device)),
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

fn get_sync_state(read_result: Option<ReadResult>, expected: &config::Device) -> SyncState {
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
                    || device.pin_access != expected.attributes.pin_access
                {
                    SyncState::Outdated
                } else {
                    SyncState::Ok
                }
            }
        },
    }
}
