use std::fmt::Debug;

use clap::{Parser, Subcommand};
use contract_transcode::{ContractMessageTranscoder, Value};
use scale::DecodeAll;
use scale::{Compact, Encode};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use subxt::error::{RpcError, TransactionError};
use subxt::ext::scale_encode::EncodeAsType;
use subxt::tx::{Signer, TxStatus};
use subxt::utils::MultiAddress;
use subxt::{
    backend::rpc,
    config::{substrate::H256, ExtrinsicParams},
    rpc_params,
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt::{tx, Config};
use subxt_signer::sr25519::dev;

mod did;

type Error = Box<dyn std::error::Error>;

type Balance = u128;

#[repr(u8)]
enum Event {
    BeforeFlipping = 0,
    AfterFlipping = 1,
}

impl Event {
    fn from(raw: u8) -> Event {
        match raw {
            0 => Event::BeforeFlipping,
            1 => Event::AfterFlipping,
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

#[derive(scale::Encode)]
struct ContractCallArgs {
    origin: <PolkadotConfig as Config>::AccountId,
    dest: <PolkadotConfig as Config>::AccountId,
    value: Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<Balance>,
    input_data: Vec<u8>,
}

#[derive(Debug, EncodeAsType, scale::Encode)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
struct Weight {
    #[codec(compact)]
    ref_time: u64,
    #[codec(compact)]
    proof_size: u64,
}

#[derive(EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
struct Call {
    dest: MultiAddress<<PolkadotConfig as Config>::AccountId, ()>,
    #[codec(compact)]
    value: Balance,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<Balance>>,
    data: Vec<u8>,
}

impl Call {
    fn build(self) -> subxt::tx::Payload<Self> {
        subxt::tx::Payload::new("Contracts", "call", self)
    }
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
    let app = App::new(cfg.did.clone());
    match cli.command {
        Commands::Config {} => eprint!("{}", toml::to_string_pretty(&cfg)?),
        Commands::Run {} => app.run(cfg.rpc_url).await?,
    }
    Ok(())
}

struct App {
    did: config::DID,
}

impl App {
    fn new(did: config::DID) -> Self {
        Self { did }
    }

    async fn run(&self, rpc_url: String) -> Result<(), Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(&rpc_url).await?;
        let rpc = rpc::RpcClient::from_url(rpc_url).await?;
        let mut i: usize = 0;
        if self.did.sync {
            self.call(api.clone(), rpc.clone(), &dev::alice()).await?;
        }
        if self.did.explorer {
            loop {
                let res: Result<H256, subxt::Error> =
                    rpc.request("chain_getBlockHash", rpc_params![i]).await;
                if let Err(e) = res {
                    match e {
                        subxt::Error::Serialization(_) => return Ok(()),
                        _ => return Err(e.to_string().into()),
                    }
                }
                let hash = res?;
                let block = api.blocks().at(hash).await?;
                let events = block.events().await?;
                eprintln!("found {:?} events in {:?}", events.len(), block.number());
                self.process_events(events)?;
                i += 1;
            }
        }
        Ok(())
    }

    fn process_events(&self, events: subxt::events::Events<PolkadotConfig>) -> Result<(), Error> {
        for event in events.iter() {
            let event = event?;
            if let Ok(evt) = event
                .as_root_event::<did::api::runtime_types::contracts_node_runtime::RuntimeEvent>()
            {
                self.process_event(evt)?
            }
        }
        Ok(())
    }

    fn process_event(
        &self,
        evt: did::api::runtime_types::contracts_node_runtime::RuntimeEvent,
    ) -> Result<(), Error> {
        if let did::api::runtime_types::contracts_node_runtime::RuntimeEvent::Contracts(
            did::api::runtime_types::pallet_contracts::pallet::Event::ContractEmitted {
                contract,
                data,
            },
        ) = evt
        {
            if contract != self.did.contract_address || data.is_empty() {
                return Ok(());
            }
            let event: Event = Event::from(data[0]);
            let mut buf = vec![0; data.len() - 1];
            buf.copy_from_slice(&data[1..]);
            match event {
                Event::BeforeFlipping => {
                    let data = BeforeFlipping::decode_all(&mut buf.as_slice())?;
                    eprintln!("{:?}", data);
                }
                Event::AfterFlipping => {
                    let data = AfterFlipping::decode_all(&mut buf.as_slice())?;
                    eprintln!("{:?}", data);
                }
            }
        }
        Ok(())
    }

    async fn call<T: Signer<PolkadotConfig>>(
        &self,
        api: OnlineClient<PolkadotConfig>,
        rpc: RpcClient,
        signer: &T,
    ) -> Result<(), Error> {
        let rpc_legacy: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(rpc);
        let transcoder = ContractMessageTranscoder::load("did.metadata.json")?; // todo: to config

        let val = self.get(&rpc_legacy, signer, &transcoder).await?;
        eprintln!("Value before executing: {:?}", val);
        self.flip(api, &rpc_legacy, signer, &transcoder).await?;
        let val = self.get(&rpc_legacy, signer, &transcoder).await?;
        eprintln!("Value after executing: {:?}", val);

        Ok(())
    }

    async fn flip<T, S>(
        &self,
        api: OnlineClient<T>,
        rpc_legacy: &LegacyRpcMethods<T>,
        signer: &S,
        transcoder: &ContractMessageTranscoder,
    ) -> Result<(), Error>
    where
        T: Config,
        S: tx::Signer<T>,
        <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
    {
        let message = "flip";
        let args: Vec<String> = vec![];
        let buf = transcoder.encode(message, args)?;
        let call = Call {
            dest: MultiAddress::Id(self.did.contract_address.clone()),
            gas_limit: Weight {
                ref_time: 500_000_000 * 10, // todo: calc
                proof_size: u64::MAX / 2,   // todo: calc
            },
            storage_deposit_limit: None,
            value: 0,
            data: buf,
        }
        .build();
        submit_tx(&api, rpc_legacy, &call, signer).await
    }

    async fn get<T, S>(
        &self,
        rpc_legacy: &LegacyRpcMethods<T>,
        signer: &S,
        transcoder: &ContractMessageTranscoder,
    ) -> Result<bool, Error>
    where
        T: Config,
        S: Signer<PolkadotConfig>,
    {
        const METHOD: &str = "get";
        let signer = <PolkadotConfig as Config>::AccountId::from(signer.account_id().0);
        let contract =
            <PolkadotConfig as Config>::AccountId::from(self.did.contract_address.clone());
        let args: Vec<String> = vec![];
        let buf = transcoder.encode(METHOD, args)?;
        let args_buf = ContractCallArgs {
            origin: signer,
            dest: contract.clone(),
            gas_limit: None,
            storage_deposit_limit: None,
            value: 0,
            input_data: buf,
        }
        .encode();
        let bytes = rpc_legacy.state_call("ContractsApi_call", Some(&args_buf), None).await?;
        let data: pallet_contracts_primitives::ContractExecResult<
            Balance,
            did::api::runtime_types::frame_system::EventRecord<
                did::api::runtime_types::contracts_node_runtime::RuntimeEvent,
                H256,
            >,
        > = scale::decode_from_bytes(bytes.clone().into())?;
        let data = data.result.unwrap();
        let data = transcoder.decode_return(METHOD, &mut data.data.as_ref())?;
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
}

async fn submit_tx<T, Call, Signer>(
    api: &OnlineClient<T>,
    rpc_legacy: &LegacyRpcMethods<T>,
    call: &Call,
    signer: &Signer,
) -> Result<(), Error>
where
    T: Config,
    Call: tx::TxPayload,
    Signer: tx::Signer<T>,
    <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
{
    let account_id = Signer::account_id(signer);
    let account_nonce = get_account_nonce(api, rpc_legacy, &account_id).await?;
    let mut tx = api
        .tx()
        .create_signed_with_nonce(call, signer, account_nonce, Default::default())?
        .submit_and_watch()
        .await?;
    while let Some(status) = tx.next().await {
        match status? {
            TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
                let _events = tx_in_block.wait_for_success().await?;
                return Ok(());
            }
            TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
            TxStatus::Invalid { message } => return Err(TransactionError::Invalid(message).into()),
            TxStatus::Dropped { message } => return Err(TransactionError::Dropped(message).into()),
            _ => continue,
        }
    }
    Err(RpcError::SubscriptionDropped.into())
}

async fn get_account_nonce<T>(
    client: &OnlineClient<T>,
    rpc_legacy: &LegacyRpcMethods<T>,
    account_id: &T::AccountId,
) -> Result<u64, Error>
where
    T: Config,
{
    let best_block = rpc_legacy
        .chain_get_block_hash(None)
        .await?
        .ok_or(subxt::Error::Other("best block not found".into()))?;
    let account_nonce = client.blocks().at(best_block).await?.account_nonce(account_id).await?;
    Ok(account_nonce)
}

// All provisioner config related source code is here.
mod config {
    use std::collections::HashMap;

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
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
        pub(crate) contract_address: <PolkadotConfig as subxt::Config>::AccountId,
        pub(crate) attributes: Attributes,
    }

    impl Default for DID {
        fn default() -> Self {
            Self {
                sync: true,
                explorer: true,
                contract_address: "5CY6gQExahvFwDZmNDLu4RxS5bQQGrQmV8yUFgRKbhd1tANC"
                    .parse()
                    .unwrap(),
                attributes: Attributes::default(),
            }
        }
    }

    // All fields are required attributes for every DID.
    // Only "additional" is additional.
    #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
    pub(crate) struct Attributes {
        pub(crate) data_type: String,
        pub(crate) location: String,
        pub(crate) price_access: String,
        pub(crate) pin_access: String,
        pub(crate) additional: Option<HashMap<String, toml::Value>>,
    }
}
