use dyn_clone::DynClone;
use std::time::UNIX_EPOCH;
use util::convert::TryConvert;
use util::Result;

use cosmrs::proto::traits::Message;
use mempools_api::api::{chain, Chain, CreateChainRequest, UpdateChainRequest};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

#[derive(Clone, Default)]
pub struct ChainFilter {
    pub id: Option<i32>,
}

#[tonic::async_trait]
pub trait ChainStorage: DynClone + Send + Sync + 'static {
    async fn create_chain(&self, request: &CreateChainRequest) -> Result<Chain>;
    async fn get_chains(&self, filter: ChainFilter, page: Option<u64>) -> Result<Vec<Chain>>;
    async fn update_chain(&self, request: &UpdateChainRequest) -> Result<Chain>;
}
dyn_clone::clone_trait_object!(ChainStorage);

#[tonic::async_trait]
impl ChainStorage for DatabaseConnection {
    async fn create_chain(&self, chain: &CreateChainRequest) -> Result<Chain> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let chain = chain.clone();
        let chain = db_entities::chain::ActiveModel {
            name: Set(chain.name),
            icon: Set(chain.icon),
            status: Set(chain::Status::Enabled as i32),
            chain_data: Set(hex::encode(
                chain
                    .chain_data
                    .ok_or("chain data not found")?
                    .encode_to_vec(),
            )),
            created_at: Set(now.to_string()),
            updated_at: Set(now.to_string()),
            ..Default::default()
        };

        let chain = chain.insert(self).await?;

        Ok(chain.try_convert()?)
    }

    async fn get_chains(&self, filter: ChainFilter, page: Option<u64>) -> Result<Vec<Chain>> {
        let mut query = db_entities::chain::Entity::find()
            .filter(db_entities::chain::Column::DeletedAt.is_null());

        if let Some(id) = filter.id {
            query = query.filter(db_entities::chain::Column::Id.eq(id));
        }

        let models;
        if let Some(page) = page {
            models = query.paginate(self, 20).fetch_page(page).await?;
        } else {
            models = query.all(self).await?;
        }

        Ok(models.try_convert()?)
    }

    async fn update_chain(&self, req: &UpdateChainRequest) -> Result<Chain> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let chain = req.chain.as_ref().ok_or("chain not found")?.clone();
        let chain = db_entities::chain::ActiveModel {
            id: Set(chain.id.parse::<i32>()?),
            name: Set(chain.name),
            icon: Set(chain.icon),
            status: Set(chain.status),
            chain_data: Set(hex::encode(
                chain
                    .chain_data
                    .ok_or("chain data not found")?
                    .encode_to_vec(),
            )),
            updated_at: Set(now.to_string()),
            ..Default::default()
        };

        let chain = chain.update(self).await?;

        Ok(chain.try_convert()?)
    }
}
