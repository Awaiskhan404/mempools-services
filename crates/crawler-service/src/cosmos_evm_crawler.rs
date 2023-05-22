use super::{
    cosmos_crawler::{CosmosCrawler, CosmosCrawlerStorage},
    eth_crawler::{EthCrawler, EthCrawlerStorage},
    storage::CrawlerStorage,
    CrawlChain,
};
use mempools_api::api::{
    crawler_data::CrawlerData, Chain, CosmosCrawlerData, CosmosEvmChainData, CosmosEvmCrawlerData,
    EthCrawlerData,
};

use util::{service_registry::ServiceRegistery, Result};

#[derive(Clone)]
pub struct CosmosEvmCrawler<S: CrawlerStorage> {
    pub cosmos_crawler: CosmosCrawler<CosmosEvmStorage<S>>,
    pub eth_crawler: EthCrawler<CosmosEvmStorage<S>>,
}

impl<S: CrawlerStorage> CosmosEvmCrawler<S> {
    pub async fn new(
        chain_data: CosmosEvmChainData,
        registery: ServiceRegistery,
        store: S,
    ) -> Result<Self> {
        let cosmos_crawler = CosmosCrawler::new(
            chain_data
                .cosmos_chain_data
                .ok_or("could not find cosmos chain data")?,
            registery.clone(),
            CosmosEvmStorage(store.clone()),
        )
        .await?;
        let eth_crawler = EthCrawler::new(
            chain_data
                .eth_chain_data
                .ok_or("could not find eth chain data")?,
            registery,
            CosmosEvmStorage(store),
        )
        .await?;

        Ok(Self {
            cosmos_crawler,
            eth_crawler,
        })
    }
}

#[tonic::async_trait]
impl<S: CrawlerStorage> CrawlChain for CosmosEvmCrawler<S> {
    async fn try_crawl_chain(&self, chain: &Chain) -> Result<()> {
        let h1 = self.cosmos_crawler.try_crawl_chain(chain);
        let h2 = self.eth_crawler.try_crawl_chain(chain);

        tokio::try_join!(h1, h2)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct CosmosEvmStorage<S: CrawlerStorage>(S);

#[tonic::async_trait]
impl<S: CrawlerStorage> CosmosCrawlerStorage for CosmosEvmStorage<S> {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64> {
        if let CrawlerData::CosmosEvm(data) = self.0.get_crawler_data(chain_id).await? {
            return Ok(data
                .cosmos
                .ok_or("could not find cosmos data")?
                .processed_blocks as i64);
        } else {
            return Err("unexpected crawler data".into());
        }
    }

    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()> {
        if let CrawlerData::CosmosEvm(mut data) =
            self.0
                .get_crawler_data(chain_id)
                .await
                .unwrap_or(CrawlerData::CosmosEvm(CosmosEvmCrawlerData {
                    cosmos: None,
                    ethereum: None,
                }))
        {
            data.cosmos = Some(CosmosCrawlerData {
                processed_blocks: height as u64,
            });

            self.0
                .set_crawler_data(chain_id, CrawlerData::CosmosEvm(data))
                .await?;

            return Ok(());
        } else {
            return Err("unexpected crawler data".into());
        }
    }
}

#[tonic::async_trait]
impl<S: CrawlerStorage> EthCrawlerStorage for CosmosEvmStorage<S> {
    async fn get_last_processed_block(&self, chain_id: &str) -> Result<i64> {
        if let CrawlerData::CosmosEvm(data) = self.0.get_crawler_data(chain_id).await? {
            return Ok(data
                .ethereum
                .ok_or("could not find cosmos data")?
                .processed_blocks as i64);
        } else {
            return Err("unexpected crawler data".into());
        }
    }

    async fn set_last_processed_block(&self, chain_id: &str, height: i64) -> Result<()> {
        if let CrawlerData::CosmosEvm(mut data) =
            self.0
                .get_crawler_data(chain_id)
                .await
                .unwrap_or(CrawlerData::CosmosEvm(CosmosEvmCrawlerData {
                    cosmos: None,
                    ethereum: None,
                }))
        {
            data.ethereum = Some(EthCrawlerData {
                processed_blocks: height as u64,
            });

            self.0
                .set_crawler_data(chain_id, CrawlerData::CosmosEvm(data))
                .await?;

            return Ok(());
        } else {
            return Err("unexpected crawler data".into());
        }
    }
}
