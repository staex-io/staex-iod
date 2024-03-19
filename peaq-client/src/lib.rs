use std::ops::Deref;

use peaq_gen::api::{
    peaq_did,
    peaq_rbac::{self, calls::types::fetch_role::Entity, events::FetchedUserPermissions},
};
use rand::RngCore;
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
    events::{EventDetails, Events, StaticEvent},
    ext::{codec::DecodeAll, sp_core::H256},
    rpc_params,
    tx::{Signer, TxInBlock, TxPayload, TxStatus},
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt_signer::{
    bip39::{self, Mnemonic},
    sr25519::Keypair,
};

pub use peaq_gen;

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
            .ok_or_else(|| subxt::Error::Other("last block is not found".into()))?;
        Ok(block)
    }

    pub async fn get_nonce(&self, account_id: &AccountId32) -> Result<u64, Error> {
        let last_block = self.get_last_block().await?;
        let account_nonce = self
            .api
            .blocks()
            .at(last_block.block.header.hash())
            .await?
            .account_nonce(account_id)
            .await?;
        Ok(account_nonce)
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

    pub async fn submit_tx<Call: TxPayload, S: Signer<PolkadotConfig>>(
        &self,
        call: &Call,
        signer: &S,
    ) -> Result<TxInBlock<PolkadotConfig, OnlineClient<PolkadotConfig>>, Error> {
        let account_id = signer.account_id();
        let account_nonce = self.get_nonce(&account_id).await?;
        let mut tx = self
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
}

pub struct SignerClient {
    client: Client,
    keypair: Keypair,
}

impl SignerClient {
    pub async fn new(rpc_url: &str, keypair: Keypair) -> Result<Self, Error> {
        let client = Client::new(rpc_url).await?;
        Ok(Self { client, keypair })
    }

    pub async fn transfer(&self, amount: u128, address: AccountId32) -> Result<(), Error> {
        self.client.transfer(amount, address, &self.keypair).await
    }

    pub async fn submit_tx<Call: TxPayload>(
        &self,
        call: &Call,
    ) -> Result<TxInBlock<PolkadotConfig, OnlineClient<PolkadotConfig>>, Error> {
        self.client.submit_tx(call, &self.keypair).await
    }

    pub fn did(&self) -> DID {
        DID {
            client: &self.client,
            signer_client: self,
            peaq_did_api: peaq_did::calls::TransactionApi {},
        }
    }

    pub fn rbac(&self) -> RBAC {
        RBAC {
            client: &self.client,
            signer_client: self,
            peaq_rbac_api: peaq_rbac::calls::TransactionApi {},
        }
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

/// RBAC structure contains methods to interact with PEAQ DID pallet.
pub struct DID<'a> {
    client: &'a Client,
    signer_client: &'a SignerClient,
    peaq_did_api: peaq_did::calls::TransactionApi,
}

impl<'a> DID<'a> {
    pub async fn add_attribute(&self, name: &str, value: Vec<u8>) -> Result<(), Error> {
        let call = self.peaq_did_api.add_attribute(
            self.signer_client.address(),
            name.as_bytes().to_vec(),
            value,
            None,
        );
        self.signer_client.submit_tx(&call).await?;
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
        let call = self
            .peaq_did_api
            .read_attribute(self.signer_client.address(), name.as_bytes().to_vec());
        let tx = self.signer_client.submit_tx(&call).await?;
        match self.client.process_events(tx, filter).await {
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
            self.signer_client.address(),
            name.as_bytes().to_vec(),
            value,
            None,
        );
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn remove_attribute(&self, name: &str) -> Result<(), Error> {
        let call = self
            .peaq_did_api
            .remove_attribute(self.signer_client.address(), name.as_bytes().to_vec());
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }
}

pub const ENTITY_ID_LENGTH: usize = 32;

pub type RBACRecord = peaq_gen::api::runtime_types::peaq_pallet_rbac::structs::Entity<Entity>;

/// RBAC structure contains methods to interact with PEAQ RBAC pallet.
#[allow(clippy::upper_case_acronyms)]
pub struct RBAC<'a> {
    client: &'a Client,
    signer_client: &'a SignerClient,
    peaq_rbac_api: peaq_rbac::calls::TransactionApi,
}

impl<'a> RBAC<'a> {
    pub async fn add_role(&self, name: String) -> Result<Entity, Error> {
        let mut role_id = [0u8; ENTITY_ID_LENGTH];
        rand::thread_rng().fill_bytes(&mut role_id);
        let call = self.peaq_rbac_api.add_role(role_id, name.into_bytes());
        self.signer_client.submit_tx(&call).await?;
        Ok(role_id)
    }

    pub async fn add_group(&self, name: String) -> Result<Entity, Error> {
        let mut group_id = [0u8; ENTITY_ID_LENGTH];
        rand::thread_rng().fill_bytes(&mut group_id);
        let call = self.peaq_rbac_api.add_group(group_id, name.into_bytes());
        self.signer_client.submit_tx(&call).await?;
        Ok(group_id)
    }

    pub async fn add_permission(&self, name: String) -> Result<Entity, Error> {
        let mut permission_id = [0u8; ENTITY_ID_LENGTH];
        rand::thread_rng().fill_bytes(&mut permission_id);
        let call = self.peaq_rbac_api.add_permission(permission_id, name.into_bytes());
        self.signer_client.submit_tx(&call).await?;
        Ok(permission_id)
    }

    pub async fn assign_permission_to_role(
        &self,
        permission_id: Entity,
        role_id: Entity,
    ) -> Result<(), Error> {
        let call = self.peaq_rbac_api.assign_permission_to_role(permission_id, role_id);
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn assign_role_to_group(
        &self,
        role_id: Entity,
        group_id: Entity,
    ) -> Result<(), Error> {
        let call = self.peaq_rbac_api.assign_role_to_group(role_id, group_id);
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn assign_user_to_group(
        &self,
        user_id: Entity,
        group_id: Entity,
    ) -> Result<(), Error> {
        let call = self.peaq_rbac_api.assign_user_to_group(user_id, group_id);
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn fetch_user_permissions(
        &self,
        owner: AccountId32,
        user_id: Entity,
    ) -> Result<Vec<RBACRecord>, Error> {
        let call = self.peaq_rbac_api.fetch_user_permissions(owner, user_id);
        let tx = self.signer_client.submit_tx(&call).await?;
        let entities = self
            .client
            .process_events(tx, Some(filter_fetched_user_permissions))
            .await?
            .ok_or_else(|| "failed to find permission entities for user".to_string())?;
        Ok(entities)
    }
}

pub fn generate_account() -> Result<(Mnemonic, Keypair, AccountId32), Error> {
    let phrase = bip39::Mnemonic::generate(12)?;
    let keypair = Keypair::from_phrase(&phrase, None)?;
    let account_id: AccountId32 =
        <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
    Ok((phrase, keypair, account_id))
}

fn filter_fetched_user_permissions(event: EventDetails<PolkadotConfig>) -> Option<Vec<RBACRecord>> {
    if event.variant_name() == peaq_rbac::events::FetchedUserPermissions::EVENT {
        if let Ok(Some(evt)) = event.as_event::<FetchedUserPermissions>() {
            return Some(evt.0);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::str::{from_utf8, FromStr};

    use subxt::{tx::Signer, utils::AccountId32, PolkadotConfig};
    use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

    use crate::{Client, SignerClient};

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

    #[ignore = "requires mnemonic phrase"]
    #[tokio::test]
    async fn test_rbac() {
        let keypair = Keypair::from_phrase(
            &Mnemonic::from_str(
                "weather asthma become jealous hurry option canal boring hedgehog rule heavy spawn",
            )
            .unwrap(),
            None,
        )
        .unwrap();
        let owner: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);

        let user_id = [
            122, 190, 82, 250, 244, 222, 128, 103, 71, 215, 50, 122, 3, 178, 251, 167, 35, 47, 138,
            5, 239, 66, 202, 72, 78, 51, 242, 157, 60, 181, 104, 107,
        ];

        let client =
            SignerClient::new("wss://rpcpc1-qa.agung.peaq.network", keypair).await.unwrap();
        let rbac = client.rbac();

        // let permission_name = String::from("mqtt_access");
        // let role_name = String::from("accessor");
        // let group_name = String::from("subscribers");

        // eprintln!("adding permission");
        // let permission_id = rbac.add_permission(permission_name).await.unwrap();
        // let permission_id = [
        //     127, 94, 254, 32, 45, 69, 35, 251, 49, 109, 247, 200, 33, 18, 22, 77, 102, 15, 192, 71,
        //     96, 209, 246, 196, 164, 103, 14, 62, 146, 209, 102, 10,
        // ];
        // eprintln!("adding role");
        // let role_id = rbac.add_role(role_name).await.unwrap();
        // let role_id = [
        //     247, 219, 4, 253, 203, 102, 224, 92, 61, 32, 29, 208, 112, 112, 242, 221, 18, 65, 165,
        //     178, 157, 1, 108, 113, 47, 116, 38, 103, 76, 221, 66, 18,
        // ];
        // eprintln!("adding group");
        // let group_id = rbac.add_group(group_name).await.unwrap();
        // let group_id = [
        //     206, 52, 97, 3, 203, 226, 87, 176, 148, 143, 97, 150, 124, 187, 229, 241, 161, 58, 59,
        //     34, 239, 108, 88, 46, 210, 79, 197, 243, 0, 194, 249, 171,
        // ];

        // eprintln!("{:?}", permission_id);
        // eprintln!("{:?}", role_id);
        // eprintln!("{:?}", group_id);

        // eprintln!("assigning permission to role");
        // rbac.assign_permission_to_role(permission_id, role_id).await.unwrap();
        // eprintln!("assigning role to group");
        // rbac.assign_role_to_group(role_id, group_id).await.unwrap();
        // eprintln!("assigning user to group");
        // rbac.assign_user_to_group(user_id, group_id).await.unwrap();

        eprintln!("fetching user permissions");
        let records = rbac.fetch_user_permissions(owner, user_id).await.unwrap();
        assert_eq!(records.len(), 1);
        assert!(records[0].enabled);
        assert_eq!(from_utf8(&records[0].name).unwrap(), "mqtt_access");
    }
}
