use std::fmt::Debug;

use scale::DecodeAll;
use subxt::{
    backend::rpc, config::substrate::H256, rpc_params, utils::AccountId32, OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "./src/did.metadata.scale")]
mod did {}

const RPC_URL: &str = "ws://127.0.0.1:9944";
const CONTRACT_ADDRESS: &str = "5Ft8zP5g45xYVjCPAtYRki9ZJYAeFNd9GAcnMJShzHiktPob";

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

type Error = Box<dyn std::error::Error>;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = OnlineClient::<PolkadotConfig>::from_url(RPC_URL).await?;
    let rpc = rpc::RpcClient::from_url(RPC_URL).await?;
    let mut i: usize = 0;
    loop {
        let res: Result<H256, subxt::Error> =
            rpc.request("chain_getBlockHash", rpc_params![i]).await;
        if let Err(e) = res {
            match e {
                subxt::Error::Serialization(_) => break,
                _ => return Err(e.to_string().into()),
            }
        }
        let hash = res?;
        let block = api.blocks().at(hash).await?;
        let events = block.events().await?;
        process_events(events)?;
        i += 1;
    }
    Ok(())
}

fn process_events(events: subxt::events::Events<PolkadotConfig>) -> Result<(), Error> {
    for event in events.iter() {
        let event = event?;
        if let Ok(evt) = event.as_root_event::<did::Event>() {
            process_event(evt)?
        }
    }
    Ok(())
}

fn process_event(evt: did::Event) -> Result<(), Error> {
    if let did::runtime_types::contracts_node_runtime::RuntimeEvent::Contracts(
        did::runtime_types::pallet_contracts::pallet::Event::ContractEmitted { contract, data },
    ) = evt
    {
        if contract.to_string() != CONTRACT_ADDRESS || data.is_empty() {
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
