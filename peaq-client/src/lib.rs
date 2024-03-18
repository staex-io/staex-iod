use std::ops::Deref;

use peaq_gen::api::peaq_did;
use subxt::{
    backend::{
        legacy::{
            rpc_methods::{BlockDetails, SystemProperties},
            LegacyRpcMethods,
        },
        rpc::RpcClient,
    },
    blocks::Block,
    config::Header,
    error::{RpcError, TransactionError},
    events::{EventDetails, Events},
    ext::{codec::DecodeAll, sp_core::H256},
    rpc_params,
    tx::{Signer, TxInBlock, TxPayload, TxStatus},
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::Keypair;

pub type Error = Box<dyn std::error::Error>;

pub struct Client {
    api: OnlineClient<PolkadotConfig>,
    rpc: RpcClient,
    rpc_legacy: LegacyRpcMethods<PolkadotConfig>,
}

impl Client {
    pub async fn new(rpc_url: &str) -> Result<Self, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
        let rpc = RpcClient::from_url(rpc_url).await?;
        let rpc_legacy: LegacyRpcMethods<PolkadotConfig> = LegacyRpcMethods::new(rpc.clone());
        Ok(Self {
            api,
            rpc,
            rpc_legacy,
        })
    }

    pub async fn get_system_properties(&self) -> Result<SystemProperties, Error> {
        Ok(self.rpc_legacy.system_properties().await?)
    }

    pub async fn get_balance(&self, address: &AccountId32) -> Result<u128, Error> {
        let last_block = self.get_last_block().await?;
        let balance_address = peaq_gen::api::storage().system().account(address);
        let info =
            self.api.storage().at(last_block.block.header.hash()).fetch(&balance_address).await?;
        if let Some(info) = info {
            return Ok(info.data.free);
        }
        // Account is not initialized yet.
        Ok(0)
    }

    pub async fn get_block(
        &self,
        index: u64,
    ) -> Result<Option<Block<PolkadotConfig, OnlineClient<PolkadotConfig>>>, Error> {
        let res: Result<H256, subxt::Error> =
            self.rpc.request("chain_getBlockHash", rpc_params![index]).await;
        if let Err(e) = res {
            match e {
                subxt::Error::Serialization(_) => return Ok(None),
                _ => return Err(e.into()),
            }
        }
        let hash = res?;
        let block = self.api.blocks().at(hash).await?;
        Ok(Some(block))
    }

    pub async fn get_events_in_block(
        &self,
        index: u64,
    ) -> Result<Option<Events<PolkadotConfig>>, Error> {
        let block = self.get_block(index).await?;
        if let Some(block) = block {
            let events = block.events().await?;
            Ok(Some(events))
        } else {
            Ok(None)
        }
    }

    pub async fn get_last_block(&self) -> Result<BlockDetails<PolkadotConfig>, Error> {
        let block = self
            .rpc_legacy
            .chain_get_block(None)
            .await?
            .ok_or(subxt::Error::Other("last block is not found".into()))?;
        Ok(block)
    }
}

pub struct SignerClient {
    client: Client,
    keypair: Keypair,
    peaq_did_api: peaq_did::calls::TransactionApi,
}

impl SignerClient {
    pub async fn new(rpc_url: &str, keypair: Keypair) -> Result<Self, Error> {
        let client = Client::new(rpc_url).await?;
        Ok(Self {
            client,
            keypair,
            peaq_did_api: peaq_did::calls::TransactionApi {},
        })
    }

    pub async fn add_attribute(&self, name: &str, value: Vec<u8>) -> Result<(), Error> {
        let call =
            self.peaq_did_api.add_attribute(self.address(), name.as_bytes().to_vec(), value, None);
        self.submit_tx(&call, &self.keypair).await?;
        Ok(())
    }

    pub async fn read_attribute<T, F>(
        &self,
        name: &str,
        filter: Option<F>,
    ) -> Result<Option<T>, Error>
    where
        F: Fn(EventDetails<PolkadotConfig>) -> Option<T>,
    {
        let call = self.peaq_did_api.read_attribute(self.address(), name.as_bytes().to_vec());
        let tx = self.submit_tx(&call, &self.keypair).await?;
        match self.process_events(tx, filter).await {
            Ok(data) => Ok(data),
            Err(e) => {
                if let subxt::Error::Runtime(subxt::error::DispatchError::Module(e)) = e {
                    // e.as_root_error() doesn't work, so we get second byte
                    // because it contains error index.
                    let a = e.bytes()[1..2].to_vec();
                    // And decode error manually.
                    let did_err = peaq_did::Error::decode_all(&mut a.as_slice())?;
                    if let peaq_did::Error::AttributeNotFound = did_err {
                        Ok(None)
                    } else {
                        Err(e.into())
                    }
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn update_attribute(&self, name: &str, value: Vec<u8>) -> Result<(), Error> {
        let call = self.peaq_did_api.update_attribute(
            self.address(),
            name.as_bytes().to_vec(),
            value,
            None,
        );
        self.submit_tx(&call, &self.keypair).await?;
        Ok(())
    }

    pub async fn remove_attribute(&self, name: &str) -> Result<(), Error> {
        let call = self.peaq_did_api.remove_attribute(self.address(), name.as_bytes().to_vec());
        self.submit_tx(&call, &self.keypair).await?;
        Ok(())
    }

    pub async fn transfer<S>(
        &self,
        amount: u128,
        address: AccountId32,
        signer: &S,
    ) -> Result<(), Error>
    where
        S: Signer<PolkadotConfig>,
    {
        let tx = peaq_gen::api::tx()
            .balances()
            .transfer_allow_death(subxt::utils::MultiAddress::Id(address), amount);
        self.submit_tx(&tx, signer).await?;
        Ok(())
    }

    async fn submit_tx<Call: TxPayload, S: Signer<PolkadotConfig>>(
        &self,
        call: &Call,
        signer: &S,
    ) -> Result<TxInBlock<PolkadotConfig, OnlineClient<PolkadotConfig>>, Error> {
        let account_id = signer.account_id();
        let account_nonce = self.get_nonce(&account_id).await?;
        let mut tx = self
            .client
            .api
            .tx()
            .create_signed_with_nonce(call, signer, account_nonce, Default::default())?
            .submit_and_watch()
            .await?;
        while let Some(status) = tx.next().await {
            match status? {
                TxStatus::InBestBlock(tx_in_block) | TxStatus::InFinalizedBlock(tx_in_block) => {
                    return Ok(tx_in_block);
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

    async fn get_nonce(&self, account_id: &AccountId32) -> Result<u64, Error> {
        let last_block = self.client.get_last_block().await?;
        let account_nonce = self
            .client
            .api
            .blocks()
            .at(last_block.block.header.hash())
            .await?
            .account_nonce(account_id)
            .await?;
        Ok(account_nonce)
    }

    async fn process_events<T, F>(
        &self,
        tx: TxInBlock<PolkadotConfig, OnlineClient<PolkadotConfig>>,
        filter: Option<F>,
    ) -> Result<Option<T>, subxt::Error>
    where
        F: Fn(EventDetails<PolkadotConfig>) -> Option<T>,
    {
        if filter.is_none() {
            return Ok(None);
        }
        let filter = filter.unwrap();
        let events = tx.wait_for_success().await?;
        for event in events.iter() {
            let event = event?;
            if let Some(data) = filter(event) {
                return Ok(Some(data));
            }
        }
        Ok(None)
    }

    fn address(&self) -> AccountId32 {
        <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&self.keypair)
    }
}

impl Deref for SignerClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn get_token_information() {
        let client = Client::new("wss://rpcpc1-qa.agung.peaq.network").await.unwrap();
        let system_properties = client.get_system_properties().await.unwrap();
        let token_decimals: u64 =
            system_properties.get("tokenDecimals").unwrap().as_number().unwrap().as_u64().unwrap();
        let token_symbol: String = system_properties.get("tokenSymbol").unwrap().to_string();
        assert_eq!(18, token_decimals);
        assert_eq!("\"AGUNG\"", token_symbol);
    }
}
