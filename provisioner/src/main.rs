use scale::{Compact, Encode, OptionBool};
use std::fmt::Debug;
use std::str::from_utf8;

use clap::{Parser, Subcommand};
use contract_transcode::ContractMessageTranscoder;
// use did::runtime_types::sp_weights::weight_v2;
use scale::DecodeAll;
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;
use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::ext::scale_encode::EncodeAsType;
use subxt::ext::sp_core::{blake2_256, Bytes};
use subxt::tx::Signer;
use subxt::utils::{MultiAddress, Static};
use subxt::{
    backend::rpc,
    config::{polkadot, substrate::H256, DefaultExtrinsicParams, ExtrinsicParams},
    rpc_params,
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt::{blocks, tx, Config};
use subxt_signer::sr25519::dev;

// use crate::did::runtime_types::sp_weights::weight_v2::Weight;

// use crate::did_gen::api::contracts::Call;
// use crate::did_gen::api::runtime_apis::contracts_api;
// // use crate::did_gen::api::contracts::calls::types::Call;
// use crate::did_gen::api::runtime_types::sp_weights::weight_v2::Weight;

// mod did_gen;

// use crate::did::contracts::calls::types::Call;

#[subxt::subxt(runtime_metadata_path = "did.metadata.scale")]
mod did {}

type Error = Box<dyn std::error::Error>;

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

mod config {
    use std::collections::HashMap;

    use subxt::Config;
    use subxt::{utils::AccountId32, PolkadotConfig};

    #[derive(serde::Serialize, serde::Deserialize)]
    pub(crate) struct XConfig {
        pub(crate) rpc_url: String,
        pub(crate) did: DID,
    }

    impl Default for XConfig {
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
        pub(crate) contract_address: <PolkadotConfig as Config>::AccountId,
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

    /// All fields are required attributes for every DID.
    /// Only "additional" is additional.
    #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
    pub(crate) struct Attributes {
        pub(crate) data_type: String,
        pub(crate) location: String,
        pub(crate) price_access: String,
        pub(crate) pin_access: String,
        pub(crate) additional: Option<HashMap<String, toml::Value>>,
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
    let cfg: config::XConfig = || -> Result<config::XConfig, Error> {
        let buf = match std::fs::read_to_string(cli.config) {
            Ok(buf) => buf,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(config::XConfig::default())
            }
            Err(e) => return Err(e.into()),
        };
        let cfg: config::XConfig = toml::from_str(&buf)?;
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
            self.call(api.clone(), self.did.contract_address.clone(), &dev::alice()).await?;
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
                // self.process_events(events)?;
                i += 1;
            }
        }
        Ok(())
    }

    // fn process_events(&self, events: subxt::events::Events<PolkadotConfig>) -> Result<(), Error> {
    //     for event in events.iter() {
    //         let event = event?;
    //         if let Ok(evt) = event.as_root_event::<did::Event>() {
    //             self.process_event(evt)?
    //         }
    //     }
    //     Ok(())
    // }

    // fn process_event(&self, evt: did::Event) -> Result<(), Error> {
    //     if let did::runtime_types::contracts_node_runtime::RuntimeEvent::Contracts(
    //         did::runtime_types::pallet_contracts::pallet::Event::ContractEmitted { contract, data },
    //     ) = evt
    //     {
    //         if contract != self.did.contract_address || data.is_empty() {
    //             return Ok(());
    //         }
    //         let event: Event = Event::from(data[0]);
    //         let mut buf = vec![0; data.len() - 1];
    //         buf.copy_from_slice(&data[1..]);
    //         match event {
    //             Event::BeforeFlipping => {
    //                 let data = BeforeFlipping::decode_all(&mut buf.as_slice())?;
    //                 eprintln!("{:?}", data);
    //             }
    //             Event::AfterFlipping => {
    //                 let data = AfterFlipping::decode_all(&mut buf.as_slice())?;
    //                 eprintln!("{:?}", data);
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    pub async fn call<T: Signer<PolkadotConfig>>(
        &self,
        api: OnlineClient<PolkadotConfig>,
        contract: <PolkadotConfig as Config>::AccountId,
        signer: &T,
    ) -> Result<H256, Error> {
        // let mut call_data = Vec::<u8>::new();
        //append the selector
        // call_data.append(&mut (&blake2_256("flip".as_bytes())[0..4]).to_vec());
        //append the arguments
        // call_data.append(&mut scale::Encode::encode(&(
        //     AccountId32::from_string(&arg1).ok()?,
        //     arg2,
        //     arg3,

        // )));

        // let api_ = contracts_api::ContractsApi {};
        // let call_ = api_.call(signer.account_id(), contract.clone(), 0, None, None, call_data);
        // let call = call_.unvalidated();

        // did_gen::Fl
        // api.tx().sign_and_submit(&all, signer, Default::default()).await?;
        // did_gen::api::tx().contracts().call(dest, value, gas_limit, storage_deposit_limit, data)
        // let params = vec![to_json_value(call_request)?];
        // let call = did::tx().contracts().call(
        //     polkadot::MultiAddress::Address32(contract.0),
        //     0,
        //     Weight {
        //         proof_size: u64::MAX / 2,
        //         ref_time: 500_000_000,
        //     },
        //     None,
        //     call_data,
        // );

        // let params = rpc_params!["ContractsApi_call", call];
        // let rpc = rpc::RpcClient::from_url("ws://127.0.0.1:9944").await?;
        // let bytes = rpc.request("state_call", params).await?;

        // let account_id = Signer::account_id(signer);
        // let nonce = api.blocks().at_latest().await?.account_nonce(&account_id).await?;
        // let account_nonce = api.blocks().at_latest().await?.account_nonce(&account_id).await?;

        let rpc = rpc::RpcClient::from_url("ws://127.0.0.1:9944").await?;

        let legacy = LegacyRpcMethods::new(rpc.clone());
        let account_id = Signer::account_id(signer);
        let account_nonce = get_account_nonce(&api, &legacy, &account_id).await?;
        // eprintln!("{}",account_nonce);

        // let signed_call =
        //     api.tx().create_signed_with_nonce(&call, signer, account_nonce, Default::default())?;
        // let hash = signed_call.submit().await?;
        // eprintln!("{:?}", hash);
        // // Ok(hash)

        // let mut call_data = Vec::<u8>::new();
        // call_data.append(&mut (&blake2_256("did::flip".as_bytes())[0..4]).to_vec());

        // let call = did::tx().contracts().call(
        //     polkadot::MultiAddress::Address32(contract.0),
        //     0,
        //     Weight {
        //         ref_time: 500_000_000_000,
        //         proof_size: u64::MAX / 2,
        //     },
        //     None,
        //     call_data,
        // );

        // let signed = api.tx().create_signed_with_nonce(&call, signer, 4, Default::default())?;
        // let hash = signed.submit().await?;
        // .wait_for_finalized_success()
        // .await
        // .unwrap();

        // let legacy = LegacyRpcMethods::new(rpc);

        let transcoder = ContractMessageTranscoder::load("did.metadata.json")?;
        let message = "flip";
        let args: Vec<String> = vec![];
        let buf = transcoder.encode(message, args)?;
        // let call = did::tx().contracts().call(
        //     MultiAddress::Id(contract.clone()),
        //     0 as u128,
        //     Weight {
        //         ref_time: 50_000_000,
        //         proof_size: u64::MAX / 10,
        //     },
        //     None,
        //     buf,
        // );
        let call = Call {
            data: buf,
            dest: MultiAddress::Id(contract.clone()),
            gas_limit: Weight {
                ref_time: 500_000_000 * 10,
                proof_size: u64::MAX / 2,
            },
            storage_deposit_limit: Some(Compact::from(1_000_000)),
            value: 0,
        }
        .build();

        let events = submit_extrinsic(&api, &legacy, &call, signer).await.unwrap();
        // eprintln!(">>> {:?}", events);

        // let signed =
        //     api.tx().create_signed_with_nonce(&call, signer, account_nonce, Default::default())?;
        // let mut tx_progress = signed.submit_and_watch().await?;
        // 'asd: loop {
        //     for progress in tx_progress.next().await {
        //         if let Ok(p) = progress {
        //             match p {
        //                 tx::TxStatus::InBestBlock(block) => break 'asd,
        //                 _ => (),
        //             }
        //         } else {
        //             break;
        //         }
        //     }
        // }
        // eprintln!("{:?}", tx_progress.extrinsic_hash());
        // let hash = tx_progress.extrinsic_hash();
        // api.tx().

        let signer = <PolkadotConfig as Config>::AccountId::from(signer.account_id().0);
        let contract = <PolkadotConfig as Config>::AccountId::from(contract.0);

        // for msg in transcoder.metadata().spec().messages() {
        //     eprintln!("{:?}", msg);
        // }

        eprintln!("{:?}", contract.to_string());
        let transcoder = ContractMessageTranscoder::load("did.metadata.json").unwrap();
        let method = "get";
        let args: Vec<String> = vec![];
        let buf = transcoder.encode(method, args)?;
        let args_buf = ContractCallArgs {
            origin: signer,
            dest: contract.clone(),
            value: 0,
            gas_limit: None,
            storage_deposit_limit: None,
            input_data: buf,
        }
        .encode();
        let bytes = legacy.state_call("ContractsApi_call", Some(&args_buf), None).await?;
        // let params = args.encode();
        // let bytes = rpc.request("ContractsApi_call", &params).await?;
        // let res = state_call(&legacy, "ContractsApi_call", args_buf).await;
        // let data: pallet_contracts_primitives::ContractResult<bool, u128, ()> =
        // scale::decode_from_bytes(bytes.into())?;
        eprintln!("{:?}", bytes);
        eprintln!("{:?}", from_utf8(&bytes));
        let data = transcoder.decode_return("get", &mut bytes.as_ref());
        eprintln!("RPC RESULT: {:?}", data);

        // let params = rpc_params!["ContractsApi_call", Bytes(args_buf)];
        // let block = api.blocks().at_latest().await?;
        // let res = legacy.state_call("ContractsApi_call", Some(&args_buf), None).await?;
        // let res = legacy.state_call("ContractsApi_call", Some(&args_buf), None).await?;
        // let res = rpc.request("state_call", params).await?;

        let data: pallet_contracts_primitives::ContractExecResult<bool, ()> =
            scale::decode_from_bytes(bytes.clone().into())?;
        eprintln!("RPC RESULT: {:?}", data);

        let data = transcoder.decode(0, &mut bytes.as_ref());
        eprintln!("RPC RESULT: {:?}", data);
        // self.rpc_call("state_call".to_string(), params).await;

        // todo!();
        let hash = H256::random();
        Ok(hash)
    }
}

async fn submit_extrinsic<T, Call, Signer>(
    client: &OnlineClient<T>,
    rpc: &LegacyRpcMethods<T>,
    call: &Call,
    signer: &Signer,
) -> core::result::Result<blocks::ExtrinsicEvents<T>, subxt::Error>
where
    T: Config,
    Call: tx::TxPayload,
    Signer: tx::Signer<T>,
    // <T::ExtrinsicParams as config::ExtrinsicParams<T>>::OtherParams: Default,
    <T::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
{
    let account_id = Signer::account_id(signer);
    let account_nonce = get_account_nonce(client, rpc, &account_id).await?;

    let mut tx = client
        .tx()
        .create_signed_with_nonce(call, signer, account_nonce, Default::default())?
        .submit_and_watch()
        .await?;

    // Below we use the low level API to replicate the `wait_for_in_block` behaviour which
    // was removed in subxt 0.33.0. See https://github.com/paritytech/subxt/pull/1237.
    //
    // We require this because we use `substrate-contracts-node` as our development node,
    // which does not currently support finality, so we just want to wait until it is
    // included in a block.
    use subxt::error::{RpcError, TransactionError};
    use tx::TxStatus;

    while let Some(status) = tx.next().await {
        match status? {
            TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
                let events = tx_in_block.wait_for_success().await.unwrap();
                return Ok(events);
            }
            TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
            TxStatus::Invalid { message } => return Err(TransactionError::Invalid(message).into()),
            TxStatus::Dropped { message } => return Err(TransactionError::Dropped(message).into()),
            _ => continue,
        }
    }
    Err(RpcError::SubscriptionDropped.into())
}

/// Return the account nonce at the *best* block for an account ID.
async fn get_account_nonce<T>(
    client: &OnlineClient<T>,
    rpc: &LegacyRpcMethods<T>,
    account_id: &T::AccountId,
) -> core::result::Result<u64, subxt::Error>
where
    T: Config,
{
    let best_block = rpc
        .chain_get_block_hash(None)
        .await?
        .ok_or(subxt::Error::Other("Best block not found".into()))?;
    let account_nonce = client.blocks().at(best_block).await?.account_nonce(account_id).await?;
    Ok(account_nonce)
}

// async fn state_call<A: scale::Encode, R: scale::Decode>(
//     rpc: &LegacyRpcMethods<PolkadotConfig>,
//     func: &str,
//     args: A,
// ) -> Result<R, Error> {
//     let params = args.encode();
//     let bytes = rpc.state_call(func, Some(&params), None).await?;
//     Ok(R::decode(&mut bytes.as_ref())?)
// }

// #[derive(scale::Encode,scale::Decode)]
// #[derive(EncodeAsType)]
// #[encode_as_type(crate_path = "subxt::ext::scale_encode")]
// struct CallRequest {
//     origin: <PolkadotConfig as Config>::AccountId,
//     dest: <PolkadotConfig as Config>::AccountId,
//     value: Balance,
//     gas_limit: Option<Weight>,
//     storage_deposit_limit: Option<Balance>,
//     input_data: Vec<u8>,
// }

/// Arguments to [`ContractRpc::call_and_get`].
#[derive(scale::Encode)]
struct ContractCallArgs {
    /// Who is singing a tx.
    pub origin: <PolkadotConfig as Config>::AccountId,
    /// Address of the contract to call.
    pub dest: <PolkadotConfig as Config>::AccountId,
    /// The balance to transfer from the `origin` to `dest`.
    pub value: Balance,
    /// The gas limit enforced when executing the constructor.
    pub gas_limit: Option<Weight>,
    /// The maximum amount of balance that can be charged from the caller to pay for the storage consumed.
    pub storage_deposit_limit: Option<Balance>,
    /// The input data to pass to the contract.
    pub input_data: Vec<u8>,
}

/// Copied from `sp_weight` to additionally implement `scale_encode::EncodeAsType`.
#[derive(Debug, EncodeAsType, scale::Encode)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub(crate) struct Weight {
    #[codec(compact)]
    /// The weight of computational time used based on some reference hardware.
    ref_time: u64,
    #[codec(compact)]
    /// The weight of storage space used by proof of validity.
    proof_size: u64,
}

impl From<sp_weights::Weight> for Weight {
    fn from(weight: sp_weights::Weight) -> Self {
        Self {
            ref_time: weight.ref_time(),
            proof_size: weight.proof_size(),
        }
    }
}

impl core::fmt::Display for Weight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Weight(ref_time: {}, proof_size: {})", self.ref_time, self.proof_size)
    }
}

/// A raw call to `pallet-contracts`'s `call`.
#[derive(EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub(crate) struct Call {
    dest: MultiAddress<<PolkadotConfig as Config>::AccountId, ()>,
    #[codec(compact)]
    value: Balance,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<Balance>>,
    data: Vec<u8>,
}

impl Call {
    pub fn new(
        dest: MultiAddress<<PolkadotConfig as Config>::AccountId, ()>,
        value: Balance,
        gas_limit: sp_weights::Weight,
        storage_deposit_limit: Option<Balance>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            dest,
            value,
            gas_limit: gas_limit.into(),
            storage_deposit_limit: storage_deposit_limit.map(Into::into),
            data,
        }
    }

    pub fn build(self) -> subxt::tx::Payload<Self> {
        subxt::tx::Payload::new("Contracts", "call", self)
    }
}

// #[derive(scale::Encode, scale::Decode,EncodeAsType, scale_info::TypeInfo, serde::Serialize, serde::Deserialize)]
// pub struct Balance(u128);

pub type Balance = u128;
