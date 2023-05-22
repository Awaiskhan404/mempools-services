use std::time::Duration;

use log::{info, warn};
use mempools_api::api::{crawler_data::CrawlerData, Chain, EthChainData, EthCrawlerData};

use util::{
    clients::new_eth_client,
    service_registry::{
        AlertSourceEthLog, AlertSourceEthTx, ProcessAlertSourceRequeust, ServiceRegistery,
    },
    HashString,
};
use web3::{
    transports::Http,
    types::{BlockNumber, Transaction},
};

use util::Result;

use super::{storage::CrawlerStorage, CrawlChain};

#[derive(Clone)]
pub struct EthCrawler<S: EthCrawlerStorage> {
    pub chain_data: EthChainData,
    pub registery: ServiceRegistery,
    pub store: S,
    pub client: web3::Web3<Http>,
}

impl<S: EthCrawlerStorage> EthCrawler<S> {
    pub async fn new(
        chain_data: EthChainData,
        registery: ServiceRegistery,
        store: S,
    ) -> Result<Self> {
        let client = new_eth_client(&chain_data.eth_rpc_endpoint).await?;

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
            let curr_block_height = self.client.eth().block_number().await?.as_u64();
            self.store
                .set_last_processed_block(chain_id, curr_block_height as i64)
                .await?;
        }

        Ok(())
    }

    async fn process_block(&self, chain: Chain, curr_block_height: u64) -> Result<()> {
        info!(
            "processing block {} for chain {}",
            curr_block_height, chain.id
        );

        let block = self
            .client
            .eth()
            .block_with_txs(web3::types::BlockId::Number(BlockNumber::Number(
                curr_block_height.into(),
            )))
            .await?
            .ok_or("could not find block at height")?;

        for tx in block.transactions {
            let svc = self.clone();
            let chain = chain.clone();
            tokio::spawn(async move {
                let tx_hash = tx.hash;
                if let Err(err) = svc.process_tx(chain.clone(), tx).await {
                    warn!(
                        "failed to execute tx {} in chain {} - {}",
                        tx_hash, chain.id, err
                    )
                }
            });
        }

        Ok(())
    }

    async fn process_tx(&self, chain: Chain, tx: Transaction) -> Result<()> {
        let tx_hash = tx.hash;
        info!("processing tx {} for chain {}", tx_hash, chain.id);

        let registery = self.registery.get_services().await?;
        let filter_svc = registery.filter_service;

        let tx_resp = self
            .client
            .eth()
            .transaction_receipt(tx_hash)
            .await?
            .ok_or("could not find reciept for transaction")?;

        let chain_id = chain.id.clone();
        let chain_data = self.chain_data.clone();
        let tx_hash = tx_hash.hash_string()?;

        if tx_resp.status.unwrap_or_default().as_u64() != 0 {
            for i in 0..tx_resp.logs.len() {
                let req = AlertSourceEthLog {
                    chain_id: chain_id.clone(),
                    chain_data: chain_data.clone(),
                    tx_hash: tx_hash.clone(),
                    log_index: i as u64,
                    log: tx_resp
                        .logs
                        .get(i)
                        .ok_or("could not find log at index")?
                        .clone(),
                };
                let svc = self.clone();
                tokio::spawn(async move {
                    if let Err(err) = svc.process_eth_log(req.clone()).await {
                        warn!(
                            "failed to process log {} in tx {} in chain {} - {}",
                            req.log_index, req.tx_hash, req.chain_id, err
                        )
                    }
                });
            }
        }

        let eth_tx = AlertSourceEthTx {
            chain_id,
            chain_data,
            tx_hash,
            tx,
            tx_resp,
        };

        filter_svc
            .process_alert_source(ProcessAlertSourceRequeust::EthTx(Box::new(eth_tx)))
            .await
            .ok();

        Ok(())
    }

    async fn process_eth_log(&self, req: AlertSourceEthLog) -> Result<()> {
        info!(
            "processing eth log {} in {} in chain {}",
            req.log_index, req.tx_hash, req.chain_id
        );

        let registery = self.registery.get_services().await?;
        let filter_svc = registery.filter_service;

        filter_svc
            .process_alert_source(ProcessAlertSourceRequeust::EthLog(Box::new(req)))
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl<S: EthCrawlerStorage> CrawlChain for EthCrawler<S> {
    async fn try_crawl_chain(&self, chain: &Chain) -> Result<()> {
        self.initialize_chain(&chain.id).await?;
        loop {
            info!("checking for new blocks in chain {}", chain.id);
            let latest_block_height = self.client.eth().block_number().await?.as_u64();

            let curr_block_height = self.store.get_last_processed_block(&chain.id).await? as u64;
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
                    .set_last_processed_block(&chain.id, latest_block_height as i64)
                    .await?;
            } else {
                tokio::time::sleep(Duration::from_secs(5)).await
            }
        }
    }
}

#[tonic::async_trait]
pub trait EthCrawlerStorage: Send + Sync + Clone + 'static {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64>;
    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()>;
}

#[tonic::async_trait]
impl<T: CrawlerStorage> EthCrawlerStorage for T {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64> {
        if let CrawlerData::Ethereum(data) = self.get_crawler_data(chain_id).await? {
            return Ok(data.processed_blocks as i64);
        } else {
            return Err("unexpected crawler data".into());
        }
    }

    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()> {
        self.set_crawler_data(
            chain_id,
            CrawlerData::Ethereum(EthCrawlerData {
                processed_blocks: height as u64,
            }),
        )
        .await?;

        Ok(())
    }
}
