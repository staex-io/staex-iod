use std::str::FromStr;

use clap::{Parser, Subcommand};
use config::Faucet;
use log::{info, Level};
use peaq_gen::api::peaq_did::events::AttributeRead;
use serde::{Deserialize, Serialize};
use subxt::{
    events::{EventDetails, StaticEvent},
    tx::Signer,
    utils::AccountId32,
    PolkadotConfig,
};
use subxt_signer::{bip39, sr25519::Keypair, SecretUri};

use crate::config::Config;

mod config;

const DID_ATTRIBUTE_NAME: &str = "staex-ioa";

type Error = Box<dyn std::error::Error>;

enum SyncState {
    Ok,
    Outdated,
    NotCreated,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
enum DID {
    #[serde(rename = "lowercase")]
    V1 {
        data_type: String,
        location: String,
        price_access: String,
        pin_access: String,
    },
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
    /// Create new account.
    NewAccount {},
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
        .filter_level(cfg.log_level.parse::<Level>()?.to_level_filter())
        .filter_module("rustls", log::LevelFilter::Off)
        .init();
    if let Commands::NewAccount {} = cli.command {
        let phrase = bip39::Mnemonic::generate(12)?;
        let keypair = Keypair::from_phrase(&phrase, None)?;
        let account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
        eprintln!("Seed phrase: {}", phrase);
        eprintln!("Address: {}", account_id);
        return Ok(());
    }
    let app: App = App::new(cfg).await?;
    match cli.command {
        Commands::Run {} => {
            app.run().await?;
        }
        Commands::Faucet { address } => {
            app.faucet(address).await?;
        }
        _ => (),
    }
    Ok(())
}

struct App {
    peaq_client: peaq_client::Client,
    faucet: Faucet,
    did: config::DID,
}

impl App {
    async fn new(cfg: Config) -> Result<Self, Error> {
        let keypair = get_keypair(&cfg.signer)?;
        Ok(Self {
            peaq_client: peaq_client::Client::new(&cfg.rpc_url, keypair).await?,
            faucet: cfg.faucet,
            did: cfg.did,
        })
    }

    async fn run(&self) -> Result<(), Error> {
        info!("starting to get on-chain did information");
        let did: Option<DID> =
            self.peaq_client.read_attribute::<DID, _>(DID_ATTRIBUTE_NAME, Some(filter)).await?;
        let sync_state: SyncState = {
            if let Some(did) = did {
                match did {
                    DID::V1 {
                        data_type,
                        location,
                        price_access,
                        pin_access,
                    } => {
                        if data_type != self.did.attributes.data_type
                            || location != self.did.attributes.location
                            || price_access != self.did.attributes.price_access
                            || pin_access != self.did.attributes.pin_access
                        {
                            SyncState::Outdated
                        } else {
                            SyncState::Ok
                        }
                    }
                }
            } else {
                SyncState::NotCreated
            }
        };
        match sync_state {
            SyncState::Ok => info!("on-chain did is up to date"),
            SyncState::Outdated => {
                info!("on-chain did is outdated; starting to sync it");
                let value = self.prepare_did()?;
                self.peaq_client.update_attribute(DID_ATTRIBUTE_NAME, value).await?;
            }
            SyncState::NotCreated => {
                info!("on-chain did is not created");
                let value = self.prepare_did()?;
                self.peaq_client.add_attribute(DID_ATTRIBUTE_NAME, value).await?;
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

    fn prepare_did(&self) -> Result<Vec<u8>, Error> {
        let did = DID::V1 {
            data_type: self.did.attributes.data_type.clone(),
            location: self.did.attributes.location.clone(),
            pin_access: self.did.attributes.pin_access.clone(),
            price_access: self.did.attributes.price_access.clone(),
        };
        let value = serde_json::to_vec(&did)?;
        Ok(value)
    }
}

fn get_keypair(cfg: &config::Signer) -> Result<Keypair, Error> {
    match cfg.typ {
        config::SignerType::Seed => {
            Ok(Keypair::from_phrase(&bip39::Mnemonic::from_str(&cfg.val)?, None)?)
        }
        config::SignerType::SecretUri => Ok(Keypair::from_uri(&SecretUri::from_str(&cfg.val)?)?),
    }
}

fn filter(event: EventDetails<PolkadotConfig>) -> Option<DID> {
    if event.variant_name() == AttributeRead::EVENT {
        if let Ok(Some(evt)) = event.as_event::<AttributeRead>() {
            match serde_json::from_slice(&evt.0.value) {
                Ok(did) => return Some(did),
                Err(_) => return None,
            }
        }
    }
    None
}
