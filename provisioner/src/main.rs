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

use crate::{balance::Balance, config::Config};

mod balance;
mod config;

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::upper_case_acronyms)]
struct DID {
    field_1: u8,
    field_2: String,
    field_3: u64,
    field_4: bool,
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
    env_logger::builder().filter_level(cfg.log_level.parse::<Level>()?.to_level_filter()).init();
    match cli.command {
        Commands::NewAccount {} => {
            let phrase = bip39::Mnemonic::generate(12)?;
            let keypair = Keypair::from_phrase(&phrase, None)?;
            let account_id: AccountId32 =
                <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
            eprintln!("Seed phrase: {}", phrase);
            eprintln!("Address: {}", account_id);
            return Ok(());
        }
        _ => (),
    }
    let app: App = App::new(cfg).await?;
    match cli.command {
        Commands::Run {} => {
            // app.run().await?;
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
}

impl App {
    async fn new(cfg: Config) -> Result<Self, Error> {
        let keypair = match cfg.signer.typ {
            config::SignerType::SecretUri => {
                Keypair::from_uri(&SecretUri::from_str(&cfg.signer.val)?)?
            }
            config::SignerType::Phrase => {
                Keypair::from_phrase(&bip39::Mnemonic::from_str(&cfg.signer.val)?, None)?
            }
        };
        Ok(Self {
            peaq_client: peaq_client::Client::new(&cfg.rpc_url, keypair).await?,
            faucet: cfg.faucet,
        })
    }

    async fn run() -> Result<(), Error> {
        todo!()
    }

    async fn faucet(&self, account_id: AccountId32) -> Result<(), Error> {
        let faucet = Keypair::from_uri(&SecretUri::from_str(&self.faucet.secret_uri)?)?;
        let faucet_account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&faucet);
        let raw_balance = self.peaq_client.get_balance(&faucet_account_id).await?;
        let balance = Balance::from_planck(raw_balance);
        info!("faucet balance: {}: {}", account_id, balance.as_dot());

        let raw_balance = self.peaq_client.get_balance(&account_id).await?;
        let balance = Balance::from_planck(raw_balance);
        info!("address balance before: {}", balance.as_dot());

        self.peaq_client.transfer(self.faucet.amount as u128, account_id.clone(), &faucet).await?;

        let raw_balance = self.peaq_client.get_balance(&account_id).await?;
        let balance = Balance::from_planck(raw_balance);
        info!("address balance after: {}", balance.as_dot());
        Ok(())
    }
}

// let client = peaq_client::Client::new("").await.unwrap();

// // Save DID.
// {
//     eprintln!("starting to add attribute on-chain");
//     let did = DID {
//         field_1: 69,
//         field_2: String::from("finally"),
//         field_3: 96,
//         field_4: false,
//     };
//     let value = serde_json::to_vec(&did).unwrap();
//     client.add_attribute("staex-ioa", value).await.unwrap();
// }
// // Get DID.
// {
//     eprintln!("starting to read attribute on-chain");
//     let did: Option<DID> =
//         client.read_attribute::<DID, _>("staex-ioa", Some(filter)).await.unwrap();
//     eprintln!("{:?}", did)
// }
// }

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
