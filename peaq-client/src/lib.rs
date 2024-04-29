use std::{fmt::Debug, ops::Deref};

use peaq_gen::api::{
    peaq_did,
    peaq_rbac::{self, calls::types::fetch_role::Entity},
};
use prost::Message;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use subxt::{
    backend::{
        legacy::{
            rpc_methods::{BlockDetails, SystemProperties},
            LegacyRpcMethods,
        },
        rpc::RpcClient,
    },
    blocks::Block,
    config::{Header, PolkadotExtrinsicParamsBuilder},
    error::{RpcError, TransactionError},
    events::Events,
    ext::sp_core::{
        bytes::{from_hex, to_hex},
        H256,
    },
    rpc_params,
    tx::{Signer, TxInBlock, TxPayload, TxStatus},
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::Keypair;

mod document;

pub use document::*;
pub use peaq_gen;

// We need custom error here to use it across threads.
pub struct Error(String);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Error> for Box<dyn std::error::Error> {
    fn from(value: Error) -> Self {
        value.into()
    }
}

impl<T: ToString> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone)]
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
        let query = peaq_gen::api::storage().system().account(address);
        let info = self.api.storage().at(last_block.block.header.hash()).fetch(&query).await?;
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
        let params = PolkadotExtrinsicParamsBuilder::new().nonce(account_nonce).build();
        let mut tx =
            self.api.tx().create_signed(call, signer, params).await?.submit_and_watch().await?;
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
}

#[derive(Clone)]
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

    pub fn address(&self) -> AccountId32 {
        <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&self.keypair)
    }
}

impl Deref for SignerClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Serialize, Deserialize)]
struct ReadAttributeResult {
    // We receive value as hex string.
    value: String,
}

/// RBAC structure contains methods to interact with PEAQ DID pallet.
pub struct DID<'a> {
    client: &'a Client,
    signer_client: &'a SignerClient,
    peaq_did_api: peaq_did::calls::TransactionApi,
}

impl<'a> DID<'a> {
    pub async fn add_attribute(&self, name: &str, doc: Document) -> Result<(), Error> {
        let value = doc.encode_to_vec();
        let call = self.peaq_did_api.add_attribute(
            self.signer_client.address(),
            name.as_bytes().to_vec(),
            value,
            None,
        );
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn read_attribute(
        &self,
        name: &str,
        address: AccountId32,
    ) -> Result<Option<Document>, Error> {
        let name = to_hex(name.as_bytes(), false);
        let latest_block = self.client.get_last_block().await?.block.header.hash();
        let result: Option<ReadAttributeResult> = self
            .client
            .rpc
            .request("peaqdid_readAttribute", rpc_params![address, name, latest_block])
            .await?;
        let result = match result {
            Some(result) => result,
            None => return Ok(None),
        };
        let value = from_hex(&result.value)?;
        let doc: Document = Document::decode(&*value)?;
        Ok(Some(doc))
    }

    pub async fn update_attribute(&self, name: &str, doc: Document) -> Result<(), Error> {
        let value = doc.encode_to_vec();
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

pub const ENTITY_LENGTH: usize = 32;

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub id: [u8; 32],
    pub name: Vec<u8>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct FetchUserPermissionsResult {
    #[serde(rename = "Ok")]
    permissions: Vec<Permission>,
}

/// RBAC structure contains methods to interact with PEAQ RBAC pallet.
#[allow(clippy::upper_case_acronyms)]
pub struct RBAC<'a> {
    client: &'a Client,
    signer_client: &'a SignerClient,
    peaq_rbac_api: peaq_rbac::calls::TransactionApi,
}

impl<'a> RBAC<'a> {
    pub async fn add_role(&self, name: String) -> Result<Entity, Error> {
        let mut role_id = [0u8; ENTITY_LENGTH];
        rand::thread_rng().fill_bytes(&mut role_id);
        let call = self.peaq_rbac_api.add_role(role_id, name.into_bytes());
        self.signer_client.submit_tx(&call).await?;
        Ok(role_id)
    }

    pub async fn add_group(&self, name: String) -> Result<Entity, Error> {
        let mut group_id = [0u8; ENTITY_LENGTH];
        rand::thread_rng().fill_bytes(&mut group_id);
        let call = self.peaq_rbac_api.add_group(group_id, name.into_bytes());
        self.signer_client.submit_tx(&call).await?;
        Ok(group_id)
    }

    pub async fn add_permission(&self, name: String) -> Result<Entity, Error> {
        let mut permission_id = [0u8; ENTITY_LENGTH];
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

    pub async fn unassign_user_from_group(
        &self,
        user_id: Entity,
        group_id: Entity,
    ) -> Result<(), Error> {
        let call = self.peaq_rbac_api.unassign_user_to_group(user_id, group_id);
        self.signer_client.submit_tx(&call).await?;
        Ok(())
    }

    pub async fn fetch_user_permissions(
        &self,
        owner: AccountId32,
        user_id: Entity,
    ) -> Result<Vec<Permission>, Error> {
        let latest_block = self.client.get_last_block().await?.block.header.hash();
        let result: FetchUserPermissionsResult = self
            .client
            .rpc
            .request("peaqrbac_fetchUserPermissions", rpc_params![owner, user_id, latest_block])
            .await?;
        Ok(result.permissions)
    }

    pub async fn fetch_permissions(&self, owner: AccountId32) -> Result<Vec<Permission>, Error> {
        let latest_block = self.client.get_last_block().await?.block.header.hash();
        let result: FetchUserPermissionsResult = self
            .client
            .rpc
            .request("peaqrbac_fetchPermissions", rpc_params![owner, latest_block])
            .await?;
        Ok(result.permissions)
    }
}

pub fn generate_account() -> Result<(bip39::Mnemonic, Keypair, AccountId32), Error> {
    let phrase = bip39::Mnemonic::generate(12)?;
    let keypair = Keypair::from_phrase(&phrase, None)?;
    let account_id: AccountId32 =
        <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
    Ok((phrase, keypair, account_id))
}

// You don't need to specify id for every service as we do it automatically.
pub fn new_document(account_id: AccountId32, mut services: Vec<Service>) -> Document {
    let id = format!("did:peaq:{}", account_id);
    // We create self-controlled DID.
    let controller = id.clone();
    let verification_method_id = format!("{}#keys-1", id);
    let verification_methods = vec![VerificationMethod {
        id: verification_method_id.clone(),
        r#type: VerificationType::Sr25519VerificationKey2020.into(),
        controller: controller.clone(),
        public_key_multibase: account_id.to_string(),
    }];
    for service in &mut services {
        service.id = format!("{}#{}", id, service.r#type)
    }
    Document {
        id,
        controller: controller.clone(),
        verification_methods,
        signature: None,
        services,
        authentications: vec![verification_method_id],
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::{from_utf8, FromStr},
        time::SystemTime,
    };

    use subxt::{tx::Signer, utils::AccountId32, PolkadotConfig};
    use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

    use crate::{
        document::{Document, Service},
        new_document, Client, SignerClient,
    };

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

    #[tokio::test]
    async fn get_latest_block() {
        let client = Client::new("wss://rpcpc1-qa.agung.peaq.network").await.unwrap();
        eprintln!("Latest block is {}", client.get_last_block().await.unwrap().block.header.number)
    }

    #[test]
    #[ignore = "run it manually to set phrase"]
    fn get_address_from_phrase() {
        let phrase = bip39::Mnemonic::parse("").unwrap();
        let keypair = Keypair::from_phrase(&phrase, None).unwrap();
        let account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
        eprintln!("Account id {}", account_id)
    }

    #[tokio::test]
    #[ignore = "run it manually to set phrase"]
    async fn get_did_attribute() {
        let phrase = bip39::Mnemonic::parse("").unwrap();
        let keypair = Keypair::from_phrase(&phrase, None).unwrap();
        let account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
        let client =
            SignerClient::new("wss://rpcpc1-qa.agung.peaq.network", keypair).await.unwrap();

        let name = format!(
            "test_{}",
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
        );
        let random_number: u128 = rand::random();
        let doc = new_document(
            account_id.clone(),
            vec![Service {
                r#type: "temperature".to_string(),
                data: random_number.to_string(),
                ..Default::default()
            }],
        );
        client.did().add_attribute(&name, doc).await.unwrap();

        let data: Document = client.did().read_attribute(&name, account_id).await.unwrap().unwrap();
        assert_eq!(1, data.services.len());
        assert_eq!("temperature", data.services[0].r#type);
        assert_eq!(random_number.to_string(), data.services[0].data);
    }

    #[tokio::test]
    #[ignore = "run it manually to set phrase"]
    async fn get_owner_permissions() {
        let phrase = bip39::Mnemonic::parse("").unwrap();
        let keypair = Keypair::from_phrase(&phrase, None).unwrap();
        let account_id: AccountId32 =
            <subxt_signer::sr25519::Keypair as Signer<PolkadotConfig>>::account_id(&keypair);
        let client =
            SignerClient::new("wss://rpcpc1-qa.agung.peaq.network", keypair).await.unwrap();
        let permissions = client.rbac().fetch_permissions(account_id).await.unwrap();
        for permission in permissions {
            eprintln!("{:?} : {:?}", from_utf8(&permission.name), permission.id);
        }
    }

    #[tokio::test]
    #[ignore = "requires manually setup mnemonic phrase"]
    async fn test_rbac() {
        let keypair = Keypair::from_phrase(&Mnemonic::from_str("").unwrap(), None).unwrap();
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
        // let role_name = String::from("mqtt_accessor");
        // let group_name = String::from("mqtt_subscribers");

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
