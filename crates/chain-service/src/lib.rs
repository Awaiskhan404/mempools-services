use mempools_api::api::{
    CreateChainRequest, CreateChainResponse, GetChainsResponse, UpdateChainRequest,
    UpdateChainResponse,
};
use util::{
    clients::new_eth_client, clients::CosmosClient, service_registry::ChainServiceInterface,
};

use util::Result;

use self::storage::{ChainFilter, ChainStorage};

pub mod storage;

#[derive(Clone)]
pub struct ChainService {
    store: Box<dyn ChainStorage>,
}

#[tonic::async_trait]
impl ChainServiceInterface for ChainService {
    async fn get_chains(&self) -> Result<GetChainsResponse> {
        let chains = self
            .store
            .get_chains(ChainFilter { id: None }, None)
            .await?;

        Ok(GetChainsResponse { chains })
    }
    /// Admin endpoints
    async fn create_chain(&self, req: &CreateChainRequest) -> Result<CreateChainResponse> {
        match req
            .clone()
            .chain_data
            .ok_or("could not find chain data")?
            .chain_data
            .ok_or("could not find chain data")?
        {
            mempools_api::api::chain_data::ChainData::CosmosChainData(data) => {
                CosmosClient::new(data.grpc_endpoint.clone()).await?;
            }
            mempools_api::api::chain_data::ChainData::CosmosEvmChainData(data) => {
                CosmosClient::new(
                    data.cosmos_chain_data
                        .ok_or("could not get cosmos chain data")?
                        .grpc_endpoint
                        .clone(),
                )
                .await?;
                new_eth_client(
                    &data
                        .eth_chain_data
                        .ok_or("could not get eth chain data")?
                        .eth_rpc_endpoint,
                )
                .await?;
            }
            mempools_api::api::chain_data::ChainData::EthChainData(data) => {
                new_eth_client(&data.eth_rpc_endpoint).await?;
            }
        }

        Ok(CreateChainResponse {
            chain: Some(self.store.create_chain(req).await?),
        })
    }
    async fn update_chain(&self, req: &UpdateChainRequest) -> Result<UpdateChainResponse> {
        Ok(UpdateChainResponse {
            chain: Some(self.store.update_chain(req).await?),
        })
    }
}

impl ChainService {
    pub fn new<S: ChainStorage>(store: S) -> Self {
        Self {
            store: Box::new(store),
        }
    }
}
