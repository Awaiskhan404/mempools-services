use std::time::Duration;

use cosmrs::proto::cosmos::{
    base::tendermint::v1beta1::{GetBlockByHeightRequest, GetLatestBlockRequest},
    tx::v1beta1::GetTxRequest,
};
use log::{info, warn};
use mempools_api::api::{crawler_data::CrawlerData, Chain, CosmosChainData, CosmosCrawlerData};

use util::{
    clients::CosmosClient,
    get_sha256_hash,
    service_registry::{
        AlertSourceCosmosMsg, AlertSourceCosmosTx, ProcessAlertSourceRequeust, ServiceRegistery,
    },
};

use util::Result;

use super::{storage::CrawlerStorage, CrawlChain};

#[derive(Clone)]
pub struct CosmosCrawler<S: CosmosCrawlerStorage> {
    pub chain_data: CosmosChainData,
    pub registery: ServiceRegistery,
    pub store: S,
    pub client: CosmosClient,
}

impl<S: CosmosCrawlerStorage> CosmosCrawler<S> {
    pub async fn new(
        chain_data: CosmosChainData,
        registery: ServiceRegistery,
        store: S,
    ) -> Result<Self> {
        let client = CosmosClient::new(chain_data.grpc_endpoint.clone()).await?;

        Ok(Self {
            chain_data,
            registery,
            store,
            client,
        })
    }

    // Create crawler - remove intialize
    pub async fn initialize_chain(&self, chain_id: &str) -> Result<()> {
        if self.store.get_last_processed_block(chain_id).await.is_err() {
            let curr_block_height = self
                .client
                .tendermint_client
                .clone()
                .get_latest_block(GetLatestBlockRequest {})
                .await?
                .get_ref()
                .clone()
                .block
                .ok_or("could not get block")?
                .header
                .ok_or("could not find block header")?
                .height;
            self.store
                .set_last_processed_block(chain_id, curr_block_height)
                .await?;
        }

        Ok(())
    }

    async fn process_block(&self, chain: Chain, curr_block_height: i64) -> Result<()> {
        info!(
            "processing block {} for chain {}",
            curr_block_height, chain.id
        );

        let block = self
            .client
            .tendermint_client
            .clone()
            .get_block_by_height(GetBlockByHeightRequest {
                height: curr_block_height,
            })
            .await?
            .get_ref()
            .clone();

        let txs = block
            .block
            .ok_or("could not get block")?
            .data
            .ok_or("could not get data for block")?
            .txs;

        for tx in txs {
            let svc = self.clone();
            let chain = chain.clone();
            let tx_hash = get_sha256_hash(&tx);
            tokio::spawn(async move {
                if let Err(err) = svc.process_tx(chain.clone(), tx_hash.clone()).await {
                    warn!(
                        "failed to execute tx {} in chain {} - {}",
                        tx_hash, chain.id, err
                    )
                }
            });
        }

        Ok(())
    }

    async fn process_tx(&self, chain: Chain, tx_hash: String) -> Result<()> {
        info!("processing tx {} for chain {}", tx_hash, chain.id);

        let registery = self.registery.get_services().await?;
        let filter_svc = registery.filter_service;

        let res = self
            .client
            .clone()
            .tx_client
            .get_tx(GetTxRequest {
                hash: tx_hash.clone(),
            })
            .await?;

        let tx = res
            .get_ref()
            .tx
            .as_ref()
            .ok_or("could not find tx in resp")?
            .clone();
        let tx_resp = res
            .get_ref()
            .tx_response
            .as_ref()
            .ok_or("could not find tx in resp")?
            .clone();

        if tx_resp.code == 0 {
            let messages = &tx.body.as_ref().ok_or("could not find tx body")?.messages;
            for i in 0..messages.len() {
                let req = AlertSourceCosmosMsg {
                    chain_id: chain.id.clone(),
                    chain_data: self.chain_data.clone(),
                    tx_hash: tx_hash.clone(),
                    msg_log: tx_resp.logs.get(i).cloned(),
                    msg_index: i as u64,
                    msg: messages
                        .get(i)
                        .ok_or("could not find msg at index")?
                        .clone(),
                };
                let svc = self.clone();
                tokio::spawn(async move {
                    let tx_hash = req.tx_hash.clone();
                    let msg_index = req.msg_index;
                    let chain_id = req.chain_id.clone();
                    if let Err(err) = svc.process_tx_msg(req).await {
                        warn!(
                            "failed to execute msg {} in tx {} in chain {} - {}",
                            msg_index, tx_hash, chain_id, err
                        )
                    }
                });
            }
        }

        filter_svc
            .process_alert_source(ProcessAlertSourceRequeust::CosmosTx(Box::new(
                AlertSourceCosmosTx {
                    chain_id: chain.id,
                    chain_data: self.chain_data.clone(),
                    tx,
                    tx_hash,
                    tx_resp,
                },
            )))
            .await?;

        Ok(())
    }

    async fn process_tx_msg(&self, req: AlertSourceCosmosMsg) -> Result<()> {
        info!(
            "processing msg {} in tx {} in chain {}",
            req.msg_index, req.tx_hash, req.chain_id
        );

        let registery = self.registery.get_services().await?;
        let filter_svc = registery.filter_service;

        filter_svc
            .process_alert_source(ProcessAlertSourceRequeust::CosmosMsg(Box::new(req)))
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl<S: CosmosCrawlerStorage> CrawlChain for CosmosCrawler<S> {
    async fn try_crawl_chain(&self, chain: &Chain) -> Result<()> {
        self.initialize_chain(&chain.id).await?;
        loop {
            info!("checking for new blocks in chain {}", chain.id);
            let latest_block_height = self
                .client
                .tendermint_client
                .clone()
                .get_latest_block(GetLatestBlockRequest {})
                .await?
                .get_ref()
                .clone()
                .block
                .ok_or("could not get block")?
                .header
                .ok_or("could not find block header")?
                .height;

            let curr_block_height = self.store.get_last_processed_block(&chain.id).await?;
            if curr_block_height != latest_block_height {
                for i in (curr_block_height + 1)..=latest_block_height {
                    let svc = self.clone();
                    let chain = chain.clone();
                    tokio::spawn(async move {
                        if let Err(err) = svc.process_block(chain.clone(), i).await {
                            warn!(
                                "failed to execute block {} in chain {} - {}",
                                i, chain.id, err
                            );
                        }
                    });
                }

                self.store
                    .set_last_processed_block(&chain.id, latest_block_height)
                    .await?;
            } else {
                tokio::time::sleep(Duration::from_secs(5)).await
            }
        }
    }
}

#[tonic::async_trait]
pub trait CosmosCrawlerStorage: Send + Sync + Clone + 'static {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64>;
    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()>;
}

#[tonic::async_trait]
impl<T: CrawlerStorage> CosmosCrawlerStorage for T {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64> {
        if let CrawlerData::Cosmos(data) = self.get_crawler_data(chain_id).await? {
            return Ok(data.processed_blocks as i64);
        } else {
            return Err("unexpected crawler data".into());
        }
    }

    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()> {
        self.set_crawler_data(
            chain_id,
            CrawlerData::Cosmos(CosmosCrawlerData {
                processed_blocks: height as u64,
            }),
        )
        .await?;

        Ok(())
    }
}
