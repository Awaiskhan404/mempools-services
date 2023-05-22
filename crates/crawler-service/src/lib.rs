use std::{collections::HashMap, time::Duration};

use eth_crawler::EthCrawler;
use log::warn;

use mempools_api::api::{chain::Status, Chain};

use tokio::task::JoinHandle;

use util::{service_registry::ServiceRegistery, Result};

use self::{
    cosmos_crawler::CosmosCrawler, cosmos_evm_crawler::CosmosEvmCrawler, storage::CrawlerStorage,
};

mod cosmos_crawler;
mod cosmos_evm_crawler;
mod eth_crawler;
mod storage;

#[derive(Clone)]
pub struct CrawlerService<S: CrawlerStorage> {
    store: S,
    registery: ServiceRegistery,
}

#[tonic::async_trait]
pub trait CrawlChain: Send + Sync {
    async fn try_crawl_chain(&self, chain: &Chain) -> Result<()>;
}

impl<S: CrawlerStorage> CrawlerService<S> {
    pub fn new(store: S, registery: ServiceRegistery) -> Self {
        Self { store, registery }
    }

    pub fn spawn_daemons(&self) {
        let svc = self.clone();
        tokio::spawn(async move { svc.crawl().await });
    }

    pub async fn crawl(&self) {
        let mut chain_crawler: HashMap<String, JoinHandle<()>> = HashMap::new();
        loop {
            if let Err(err) = self.try_crawl(&mut chain_crawler).await {
                warn!("crawler failed - {} - restarting in 10 seconds...", err);
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }

    async fn try_crawl(&self, chain_crawler: &mut HashMap<String, JoinHandle<()>>) -> Result<()> {
        let registery = self.registery.get_services().await?;
        let chain_service = registery.chain_service;

        loop {
            let chains = chain_service.get_chains().await?.chains;
            for chain in chains {
                if chain.status == Status::Enabled as i32 {
                    if !chain_crawler.contains_key(&chain.id) {
                        let crawler = self.clone();
                        let chain_id = chain.id.clone();
                        let chain = chain.clone();
                        let handle = tokio::spawn(async move { crawler.crawl_chain(chain).await });
                        chain_crawler.insert(chain_id, handle);
                    }
                } else if let Some(handle) = chain_crawler.get(&chain.id) {
                    handle.abort();
                    chain_crawler.remove(&chain.id);
                }
            }
            tokio::time::sleep(Duration::from_secs(30)).await
        }
    }

    pub async fn crawl_chain(&self, chain: Chain) {
        loop {
            if let Err(err) = self.try_crawl_chain(&chain).await {
                warn!(
                    "crawler for chain {:?} failed - {} - restarting in 10 seconds...",
                    chain, err
                );
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }

    async fn try_crawl_chain(&self, chain: &Chain) -> Result<()> {
        let registery = self.registery.clone();
        let store = self.store.clone();
        let crawler = match chain
            .chain_data
            .as_ref()
            .ok_or("could not find chain data")?
            .chain_data
            .as_ref()
            .ok_or("could not find chain data")?
            .clone()
        {
            mempools_api::api::chain_data::ChainData::CosmosChainData(c) => {
                Box::new(CosmosCrawler::new(c, registery, store).await?) as Box<dyn CrawlChain>
            }
            mempools_api::api::chain_data::ChainData::CosmosEvmChainData(c) => {
                Box::new(CosmosEvmCrawler::new(c, registery, store).await?) as Box<dyn CrawlChain>
            }
            mempools_api::api::chain_data::ChainData::EthChainData(c) => {
                Box::new(EthCrawler::new(c, registery, store).await?) as Box<dyn CrawlChain>
            }
        };

        crawler.try_crawl_chain(chain).await?;

        Ok(())
    }
}
