use std::fmt::Debug;
use std::str::from_utf8;

use clap::{Parser, Subcommand};
use contract_transcode::{ContractMessageTranscoder, Value};
use pallet_contracts_primitives::ContractExecResult;
use scale::{DecodeAll, Encode};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use subxt::error::{RpcError, TransactionError};
use subxt::events::Events;
use subxt::tx::{Signer, TxPayload, TxStatus};
use subxt::utils::MultiAddress;
use subxt::{
    backend::rpc, config::substrate::H256, rpc_params, utils::AccountId32, OnlineClient,
    PolkadotConfig,
};
use subxt_signer::sr25519::{dev, Keypair};

use crate::did::api::runtime_apis::contracts_api::types::Call;
use crate::did::api::runtime_types::contracts_node_runtime::RuntimeEvent;
use crate::did::api::runtime_types::frame_system::EventRecord;
use crate::did::api::runtime_types::pallet_contracts::pallet::Event;
use crate::did::api::TransactionApi;

mod did;

const DOT_MULTIPLIER: u128 = 10u128.pow(16);

type Error = Box<dyn std::error::Error>;

type Balance = u128;

#[repr(u8)]
enum DIDEvent {
    BeforeFlipping = 0,
    AfterFlipping = 1,
}

impl DIDEvent {
    fn from(raw: u8) -> DIDEvent {
        match raw {
            0 => DIDEvent::BeforeFlipping,
            1 => DIDEvent::AfterFlipping,
            _ => unimplemented!(),
        }
    }
}

#[derive(scale::Decode, Debug)]
struct BeforeFlipping {
    _from: AccountId32,
    _field1: u64,
    _field2: String,
    _field3: String,
}

#[derive(scale::Decode, Debug)]
struct AfterFlipping {
    _from: AccountId32,
    _field1: u64,
    _field2: String,
    _field3: bool,
}

#[derive(scale::Encode, scale::Decode, Debug)]
struct Weight {
    #[codec(compact)]
    ref_time: u64,
    #[codec(compact)]
    proof_size: u64,
}

impl Weight {
    fn new(ref_time: u64, proof_size: u64) -> Self {
        Self {
            ref_time,
            proof_size,
        }
    }
}

impl From<Weight> for crate::did::api::runtime_types::sp_weights::weight_v2::Weight {
    fn from(value: Weight) -> Self {
        Self {
            ref_time: value.ref_time,
            proof_size: value.proof_size,
        }
    }
}

#[derive(Debug)]
struct DryRunResult {
    data: Value,
    gas_required: Weight,
}

/// Command line utility to interact with StaexIoD provisioner.
#[derive(Parser)]
#[clap(name = "provisioner")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Config file path.
    #[arg(short, long)]
    #[arg(default_value = "config.toml")]
    config: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show default config for provisioner
    Config {},
    /// Run provisioner.
    Run {},
    /// Create new account.
    NewAccount {},
    /// Faucet your account from dev accounts.
    Faucet { address: AccountId32 },
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
    // Offline commands.
    let mut offline_was_executed = true;
    match cli.command {
        Commands::NewAccount {} => {
            let phrase = bip39::Mnemonic::generate(12)?;
            let pair = Keypair::from_phrase(&phrase, None)?;
            let account_id: AccountId32 =
                <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&pair);
            eprintln!("Seed phrase: {:?}", phrase.to_string());
            eprintln!("Address: {:?}", account_id.to_string());
        }
        _ => offline_was_executed = false,
    }
    if offline_was_executed {
        return Ok(());
    }
    let app: App = App::new(cfg.did, cfg.rpc_url).await?;
    // Online commands.
    match cli.command {
        Commands::Run {} => {
            app.run().await?;
        }
        Commands::Faucet { address } => {
            let balance = app.get_balance(&address).await?;
            eprintln!("Balance: {:?} DOT", balance / DOT_MULTIPLIER);

            let tx = crate::did::api::tx()
                .balances()
                .transfer_allow_death(MultiAddress::Id(address.clone()), 10 * DOT_MULTIPLIER);
            app.submit_tx(&tx, &dev::alice()).await?;

            // todo: do we need to finalize block manually to see changed balance?
            // let bytes = app.rpc.request("engine_finalizeBlock", rpc_params![]).await?;
            // eprintln!("{:?}", bytes);

            let balance = app.get_balance(&address).await?;
            eprintln!("Balance: {:?} DOT", balance / DOT_MULTIPLIER);
        }
        _ => (),
    }
    Ok(())
}

struct App {
    did: config::DID,
    api: OnlineClient<PolkadotConfig>,
    rpc: RpcClient,
    rpc_legacy: LegacyRpcMethods<PolkadotConfig>,
    transcoder: ContractMessageTranscoder,
}

impl App {
    async fn new(did: config::DID, rpc_url: String) -> Result<Self, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(&rpc_url).await?;
        let rpc = rpc::RpcClient::from_url(rpc_url).await?;
        let rpc_legacy: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(rpc.clone());
        let transcoder = ContractMessageTranscoder::load(did.metadata_path.clone())?;
        Ok(Self {
            did,
            api,
            rpc,
            rpc_legacy,
            transcoder,
        })
    }

    async fn run(&self) -> Result<(), Error> {
        let mut i: usize = 0;
        if self.did.sync {
            let signer = dev::alice();
            self.sync(&signer).await?;
        }
        if self.did.explorer {
            loop {
                let res: Result<H256, subxt::Error> =
                    self.rpc.request("chain_getBlockHash", rpc_params![i]).await;
                if let Err(e) = res {
                    match e {
                        subxt::Error::Serialization(_) => return Ok(()),
                        _ => return Err(e.to_string().into()),
                    }
                }
                let hash = res?;
                let block = self.api.blocks().at(hash).await?;
                let events = block.events().await?;
                eprintln!("found {:?} events in {:?}", events.len(), block.number());
                self.process_events(&events)?;
                i += 1;
            }
        }
        Ok(())
    }

    fn process_events(&self, events: &Events<PolkadotConfig>) -> Result<(), Error> {
        for event in events.iter() {
            let event = event?;
            if let Ok(evt) = event.as_root_event::<RuntimeEvent>() {
                self.process_event(evt)?
            }
        }
        Ok(())
    }

    fn process_event(&self, evt: RuntimeEvent) -> Result<(), Error> {
        if let RuntimeEvent::Contracts(Event::ContractEmitted { contract, data }) = evt {
            if contract != self.did.contract_address || data.is_empty() {
                return Ok(());
            }
            let event: DIDEvent = DIDEvent::from(data[0]);
            let mut buf = vec![0; data.len() - 1];
            buf.copy_from_slice(&data[1..]);
            match event {
                DIDEvent::BeforeFlipping => {
                    let data = BeforeFlipping::decode_all(&mut buf.as_slice())?;
                    eprintln!("{:?}", data);
                }
                DIDEvent::AfterFlipping => {
                    let data = AfterFlipping::decode_all(&mut buf.as_slice())?;
                    eprintln!("{:?}", data);
                }
            }
        }
        Ok(())
    }

    async fn sync<S: Signer<PolkadotConfig>>(&self, signer: &S) -> Result<(), Error> {
        let val = self.get(signer.account_id()).await?;
        eprintln!("Value before executing: {:?}", val);
        self.flip(signer).await?;
        let val = self.get(signer.account_id()).await?;
        eprintln!("Value after executing: {:?}", val);
        Ok(())
    }

    async fn get_balance(&self, address: &AccountId32) -> Result<u128, Error> {
        let balance_address = crate::did::api::storage().system().account(address);
        let info = self.api.storage().at_latest().await?.fetch(&balance_address).await?;
        if let Some(info) = info {
            return Ok(info.data.free);
        }
        eprintln!("Account is not initialized?");
        Ok(0)
    }

    async fn flip<S: Signer<PolkadotConfig>>(&self, signer: &S) -> Result<(), Error> {
        let message = "flip";
        let input_data_args: Vec<String> = vec![];
        let dry_run_res =
            self.dry_run(message, input_data_args.clone(), signer.account_id()).await?;
        let data = self.transcoder.encode(message, input_data_args)?;
        let call = (TransactionApi {}).contracts().call(
            MultiAddress::Id(self.did.contract_address.clone()),
            0,
            dry_run_res.gas_required.into(),
            None,
            data,
        );
        self.submit_tx(&call, signer).await
    }

    async fn get(&self, sender: AccountId32) -> Result<bool, Error> {
        const MESSAGE: &str = "get";
        let input_data_args: Vec<String> = vec![];
        let data = self.dry_run(MESSAGE, input_data_args, sender).await?.data;
        match data {
            Value::Tuple(t) => {
                if t.values().count() != 1 {
                    return Err(format!("unexpected values count: {}", t.values().count()).into());
                }
                let value = t.values().last().ok_or::<&str>("last value is not found")?;
                match value {
                    Value::Bool(b) => Ok(*b),
                    _ => Err("unexpected response: value in tuple is not bool".into()),
                }
            }
            _ => Err("unexpected response: value is not tuple".into()),
        }
    }

    async fn submit_tx<Call: TxPayload, S: Signer<PolkadotConfig>>(
        &self,
        call: &Call,
        signer: &S,
    ) -> Result<(), Error> {
        let account_id = signer.account_id();
        let account_nonce = self.get_account_nonce(&account_id).await?;
        let mut tx = self
            .api
            .tx()
            .create_signed_with_nonce(call, signer, account_nonce, Default::default())?
            .submit_and_watch()
            .await?;
        while let Some(status) = tx.next().await {
            match status? {
                TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
                    let events = tx_in_block.wait_for_success().await?;
                    self.process_events(events.all_events_in_block())?;
                    return Ok(());
                }
                TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
                TxStatus::Invalid { message } => {
                    return Err(TransactionError::Invalid(message).into())
                }
                TxStatus::Dropped { message } => {
                    return Err(TransactionError::Dropped(message).into())
                }
                _ => continue,
            }
        }
        Err(RpcError::SubscriptionDropped.into())
    }

    async fn get_account_nonce(&self, account_id: &AccountId32) -> Result<u64, Error> {
        let best_block = self
            .rpc_legacy
            .chain_get_block_hash(None)
            .await?
            .ok_or(subxt::Error::Other("best block not found".into()))?;
        let account_nonce =
            self.api.blocks().at(best_block).await?.account_nonce(account_id).await?;
        Ok(account_nonce)
    }

    async fn dry_run(
        &self,
        message: &str,
        input_data_args: Vec<String>,
        sender: AccountId32,
    ) -> Result<DryRunResult, Error> {
        let input_data = self.transcoder.encode(message, input_data_args)?;
        let args = Call {
            origin: sender,
            dest: self.did.contract_address.clone(),
            gas_limit: None,
            storage_deposit_limit: None,
            value: 0,
            input_data,
        }
        .encode();
        let bytes = self.rpc_legacy.state_call("ContractsApi_call", Some(&args), None).await?;
        let exec_res: ContractExecResult<Balance, EventRecord<RuntimeEvent, H256>> =
            scale::decode_from_bytes(bytes.clone().into())?;
        let exec_res_data = exec_res.result.unwrap();
        let data = self.transcoder.decode_return(message, &mut exec_res_data.data.as_ref())?;
        eprintln!("Message logs: {}: {:?}", message, from_utf8(&exec_res.debug_message).unwrap());
        Ok(DryRunResult {
            data,
            gas_required: Weight::new(
                exec_res.gas_required.ref_time(),
                exec_res.gas_required.proof_size(),
            ),
        })
    }
}

// All provisioner config related source code is here.
mod config {
    use std::collections::HashMap;

    use subxt::utils::AccountId32;

    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    pub(crate) struct Config {
        pub(crate) rpc_url: String,
        pub(crate) did: DID,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                rpc_url: "ws://127.0.0.1:9944".to_string(),
                did: DID::default(),
            }
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    pub(crate) struct DID {
        pub(crate) sync: bool,
        pub(crate) explorer: bool,
        pub(crate) contract_address: AccountId32,
        pub(crate) metadata_path: String,
        pub(crate) attributes: Attributes,
    }

    impl Default for DID {
        fn default() -> Self {
            Self {
                sync: true,
                explorer: true,
                contract_address: "5CC6P6tLiRAao554Mkq7LnXeD4AyfcamMPrm64BdH7kdmx9W"
                    .parse()
                    .unwrap(),
                metadata_path: "did.metadata.json".to_string(),
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
}
