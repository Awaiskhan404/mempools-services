use cosmrs::proto::traits::Message;
use mempools_api::api::crawler_data::CrawlerData;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use util::Result;

#[tonic::async_trait]
pub trait CrawlerStorage: Clone + Send + Sync + 'static {
    async fn get_crawler_data(&self, chain_id: &str) -> Result<CrawlerData>;
    async fn set_crawler_data(&self, chain_id: &str, data: CrawlerData) -> Result<()>;
}

#[tonic::async_trait]
impl CrawlerStorage for DatabaseConnection {
    async fn get_crawler_data(&self, chain_id: &str) -> Result<CrawlerData> {
        let row = db_entities::crawler::Entity::find()
            .filter(db_entities::crawler::Column::ChainId.eq(chain_id.parse::<i32>()?))
            .one(self)
            .await?
            .ok_or("could not find chain")?;

        let data = mempools_api::api::CrawlerData::decode(hex::decode(row.data)?.as_slice())?;

        Ok(data.crawler_data.ok_or("could not find crawler data")?)
    }

    async fn set_crawler_data(&self, chain_id: &str, data: CrawlerData) -> Result<()> {
        let row = db_entities::crawler::Entity::find()
            .filter(db_entities::crawler::Column::ChainId.eq(chain_id.parse::<i32>()?))
            .one(self)
            .await?;

        let data = hex::encode(
            mempools_api::api::CrawlerData {
                crawler_data: Some(data),
            }
            .encode_to_vec(),
        );

        if let Some(row) = row {
            let mut row = row.into_active_model();
            row.data = Set(data);
            row.update(self).await?;
        } else {
            let row = db_entities::crawler::ActiveModel {
                chain_id: Set(chain_id.parse::<i32>()?),
                data: Set(data),
                ..Default::default()
            };
            row.insert(self).await?;
        }

        Ok(())
    }
}
