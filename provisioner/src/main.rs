use std::{collections::HashMap, str::FromStr, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{Parser, Subcommand};
use log::{debug, error, info, trace, warn, Level, LevelFilter};
use peaq_client::{generate_account, peaq_gen::api::peaq_did::events::AttributeRead};
use serde::{Deserialize, Serialize};
use subxt::{
    config::Header,
    events::{EventDetails, StaticEvent},
    tx::Signer,
    utils::AccountId32,
    PolkadotConfig,
};
use subxt_signer::{
    bip39::{self},
    sr25519::Keypair,
    SecretUri,
};
use tokio::{
    sync::{mpsc, watch},
    time::timeout,
};

use crate::config::Config;

mod child_process;
mod config;
mod example_app;
mod indexer;
mod rbac;
mod staex_mcc;

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
    price_pin: f64,
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
    /// Grant access to provisioner to the new user.
    GrantAccess {
        /// User address.
        address: AccountId32,
    },
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
    if let Commands::Config {} = cli.command {
        eprint!("{}", toml::to_string_pretty(&config::Config::default())?);
        return Ok(());
    }
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
        Commands::Run {} => {
            let app: App = App::new(cfg).await?;
            let (stop_s, stop_r) = watch::channel(());
            tokio::spawn(async move {
                if let Err(e) = app.run(stop_r).await {
                    error!("failed to run application: {e}")
                }
            });
            tokio::signal::ctrl_c().await?;
            debug!("received termination signal");
            stop_s.send(())?;
            if let Err(e) = timeout(Duration::from_secs(10), stop_s.closed()).await {
                error!("failed to stop staex mcc and example app because of timeout: {}", e);
            }
            info!("everything was stopped successfully");
        }
        Commands::Indexer {} => {
            indexer::run(cfg).await?;
            tokio::signal::ctrl_c().await?;
        }
        Commands::NewAccount {} => {
            let (phrase, _, account_id) = generate_account()?;
            eprintln!("Phrase: {}", phrase);
            eprintln!("Address: {}", account_id);
        }
        Commands::GrantAccess { address } => {
            let app: App = App::new(cfg).await?;
            app.grant_access(address).await?;
        }
        Commands::SelfRemove {} => {
            let app: App = App::new(cfg).await?;
            app.self_remove().await?;
        }
        Commands::Faucet { address } => {
            let app: App = App::new(cfg).await?;
            app.faucet(address).await?;
        }
        _ => (),
    };
    Ok(())
}

struct App {
    cfg: config::Config,
    peaq_client: peaq_client::SignerClient,
}

impl App {
    async fn new(cfg: Config) -> Result<Self, Error> {
        let keypair = get_keypair(&cfg.signer)?;
        let peaq_client = peaq_client::SignerClient::new(&cfg.rpc_url, keypair).await?;
        Ok(Self { cfg, peaq_client })
    }

    async fn run(&self, stop_r: watch::Receiver<()>) -> Result<(), Error> {
        self.sync_did().await?;
        info!("device is synchronize");
        rbac::init_rbac(&self.cfg.rbac, &self.peaq_client).await?;
        info!("rbac is initialized");
        info!("starting staex mcc and example app");
        self.start(stop_r).await
    }

    async fn sync_did(&self) -> Result<(), Error> {
        if !self.cfg.device.sync {
            return Ok(());
        }
        let last_block = self.peaq_client.get_last_block().await?;
        info!(
            "starting to get on-chain device information starting from {} block",
            last_block.block.header.number()
        );
        let read_result: Option<ReadResult> = self
            .peaq_client
            .did()
            .read_attribute::<ReadResult, _>(DEVICE_ATTRIBUTE_NAME, Some(filter))
            .await?;
        let sync_state = get_sync_state(read_result, &self.cfg.device);
        match sync_state {
            SyncState::Ok => {
                info!("on-chain device is up to date");
                if self.cfg.device.force {
                    warn!("force sync is enabled; starting to sync it");
                    let value = self.prepare_device()?;
                    self.peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                    info!("successfully updated on-chain device");
                }
            }
            SyncState::Outdated => {
                info!("on-chain device is outdated; starting to sync it");
                let value = self.prepare_device()?;
                self.peaq_client.did().update_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                info!("successfully updated on-chain device");
            }
            SyncState::NotCreated => {
                info!("on-chain device is not created");
                let value = self.prepare_device()?;
                self.peaq_client.did().add_attribute(DEVICE_ATTRIBUTE_NAME, value).await?;
                info!("successfully created on-chain device");
            }
        }
        Ok(())
    }

    async fn start(&self, stop_r: watch::Receiver<()>) -> Result<(), Error> {
        let (restart_s, restart_r) = mpsc::channel::<()>(1);
        let stop_r_ = stop_r.clone();
        tokio::spawn(async move {
            if let Err(e) = staex_mcc::run_staex_mcc(restart_r, stop_r_).await {
                error!("failed to run staex mcc: {e}")
            }
        });
        let stop_r_ = stop_r.clone();
        tokio::spawn(async move {
            if let Err(e) = example_app::run_example_app(stop_r_).await {
                error!("failed to run example app: {e}")
            }
        });
        let cfg_rbac = self.cfg.rbac.clone();
        let peaq_client = self.peaq_client.clone();
        tokio::spawn(async move {
            if let Err(e) = rbac::sync_rbac(cfg_rbac, peaq_client, restart_s, stop_r).await {
                error!("failed to run sync rbac: {e}")
            }
        });
        Ok(())
    }

    async fn faucet(&self, account_id: AccountId32) -> Result<(), Error> {
        let signer = get_keypair(&self.cfg.faucet.signer)?;
        let faucet_account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&signer);
        let balance = self.peaq_client.get_balance(&faucet_account_id).await?;
        info!("faucet balance: {}: {}", faucet_account_id, balance,);

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance before: {}", balance);

        self.peaq_client.transfer(self.cfg.faucet.amount as u128, account_id.clone()).await?;

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance after: {}", balance);
        Ok(())
    }

    async fn grant_access(&self, address: AccountId32) -> Result<(), Error> {
        let group_id = BASE64_STANDARD.decode(&self.cfg.rbac.group_id)?;
        let group_id = vec_to_bytes(group_id)?;
        trace!("address array is {:?}", address.0);
        trace!("group id array is {:?}", group_id);
        self.peaq_client.rbac().assign_user_to_group(address.0, group_id).await
    }

    async fn self_remove(&self) -> Result<(), Error> {
        info!("starting to do self-remove");
        self.peaq_client.did().remove_attribute(DEVICE_ATTRIBUTE_NAME).await
    }

    fn prepare_device(&self) -> Result<Vec<u8>, Error> {
        let device = Device::V1(DeviceV1 {
            data_type: self.cfg.device.attributes.data_type.clone(),
            location: self.cfg.device.attributes.location.clone(),
            price_pin: self.cfg.device.attributes.price_pin,
            price_access: self.cfg.device.attributes.price_access,
            additional: self.cfg.device.attributes.additional.clone(),
        });
        let value = serde_json::to_vec(&device)?;
        Ok(value)
    }
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
                    || device.price_pin != expected.attributes.price_pin
                {
                    SyncState::Outdated
                } else {
                    SyncState::Ok
                }
            }
        },
    }
}

pub(crate) fn vec_to_bytes<T, const N: usize>(v: Vec<T>) -> Result<[T; N], Error> {
    Ok(v.try_into().map_err(|v: Vec<T>| {
        format!(
            "failed to convert vector to array because vector length is {} but expected {}",
            v.len(),
            N
        )
    })?)
}
