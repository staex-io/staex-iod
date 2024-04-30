use std::{str::FromStr, time::Duration};

use clap::{Parser, Subcommand};
use log::{debug, error, info, Level, LevelFilter};
use peaq_client::generate_account;
use subxt::utils::AccountId32;
use subxt_signer::{
    bip39::{self, Mnemonic},
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
mod did;
mod example_app;
mod indexer;
mod rbac;
mod staex_mcc;

pub(crate) type Error = Box<dyn std::error::Error>;

/// Command line utility to interact with StaexIoD provisioner.
#[derive(Parser)]
#[clap(name = "provisioner")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Provisioner config file path.
    #[arg(short, long)]
    #[arg(default_value = "config.toml")]
    config: String,
    /// Set RPC URL for PEAQ network.
    #[arg(short, long)]
    #[arg(default_value = "wss://rpcpc1-qa.agung.peaq.network")]
    rpc_url: String,
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
    /// Revoke access.
    RevokeAccess {
        /// User address.
        address: AccountId32,
    },
    /// Update client DID attributes.
    /// This command can be used to map StaexMCC id with PEAQ address.
    UpdateClient {
        /// Secret phrase.
        phrase: Mnemonic,
        /// StaexMCC identifier.
        staex_mcc_id: String,
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
    match cli.command {
        Commands::Config {} => {
            eprint!("{}", toml::to_string_pretty(&config::Config::default())?);
            return Ok(());
        }
        Commands::UpdateClient {
            phrase,
            staex_mcc_id,
        } => return did::update_client_info(&cli.rpc_url, &phrase, staex_mcc_id).await,
        _ => (),
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
            match timeout(Duration::from_secs(10), stop_s.closed()).await {
                Ok(_) => info!("everything was stopped successfully"),
                Err(e) => {
                    error!("failed to stop staex mcc and example app because of timeout: {}", e)
                }
            }
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
            rbac::grant_access(&app.peaq_client, address, &app.cfg.rbac.group_id).await?;
        }
        Commands::RevokeAccess { address } => {
            let app: App = App::new(cfg).await?;
            rbac::revoke_access(&app.peaq_client, address, &app.cfg.rbac.group_id).await?;
        }
        Commands::SelfRemove {} => {
            let app: App = App::new(cfg).await?;
            did::self_remove(app.peaq_client).await?;
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
        did::sync_did(&self.cfg.device, &self.peaq_client).await?;
        info!("device is synchronize");
        rbac::init_rbac(&self.cfg.rbac, &self.peaq_client).await?;
        info!("rbac is initialized");
        info!("starting staex mcc and example app");
        self.start(stop_r).await
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
        let faucet_account_id: AccountId32 = signer.public_key().to_account_id();
        let balance = self.peaq_client.get_balance(&faucet_account_id).await?;
        info!("faucet balance: {}: {}", faucet_account_id, balance,);

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance before: {}", balance);

        self.peaq_client.transfer(self.cfg.faucet.amount as u128, account_id.clone()).await?;

        let balance = self.peaq_client.get_balance(&account_id).await?;
        info!("address balance after: {}", balance);
        Ok(())
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

fn get_keypair(cfg: &config::Signer) -> Result<Keypair, Error> {
    match cfg.typ {
        config::SignerType::Phrase => {
            Ok(Keypair::from_phrase(&bip39::Mnemonic::from_str(&cfg.val)?, None)?)
        }
        config::SignerType::SecretUri => Ok(Keypair::from_uri(&SecretUri::from_str(&cfg.val)?)?),
    }
}
