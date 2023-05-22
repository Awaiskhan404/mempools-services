use std::time::UNIX_EPOCH;

use dyn_clone::DynClone;
use util::{service_registry::UserDestinationFilter, Result};

use cosmrs::proto::traits::Message;
use mempools_api::api::{
    user_destination, DestinationRegistrationProposal, InitiateDestinationRegistrationRequest,
    UserDestination,
};
use rand::{rngs::StdRng, Rng};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use util::convert::TryConvert;

#[tonic::async_trait]
pub trait DestinationStorage: DynClone + Send + Sync + 'static {
    async fn create_destination_registration_proposal(
        &self,
        request: &InitiateDestinationRegistrationRequest,
    ) -> Result<DestinationRegistrationProposal>;
    async fn get_destination_registration_proposal_by_id(
        &self,
        id: String,
    ) -> Result<DestinationRegistrationProposal>;
    async fn create_user_destination(
        &self,
        request: &DestinationRegistrationProposal,
    ) -> Result<UserDestination>;
    async fn get_user_destinations(
        &self,
        filter: UserDestinationFilter,
        page: Option<u64>,
    ) -> Result<Vec<UserDestination>>;
    async fn update_user_destination(&self, dest: UserDestination) -> Result<UserDestination>;
    async fn delete_user_destination(&self, id: i32) -> Result<()>;
}
dyn_clone::clone_trait_object!(DestinationStorage);

#[tonic::async_trait]
impl DestinationStorage for DatabaseConnection {
    async fn create_destination_registration_proposal(
        &self,
        request: &InitiateDestinationRegistrationRequest,
    ) -> Result<DestinationRegistrationProposal> {
        let mut rng: StdRng = rand::SeedableRng::from_entropy();
        let confirmation_code = rng.gen_range(1000..9999);

        let drp = db_entities::destination_registration_proposal::ActiveModel {
            user_id: Set(request.user_id.clone()),
            destination: Set(hex::encode(
                request
                    .destination
                    .as_ref()
                    .ok_or("could not find destination")?
                    .encode_to_vec(),
            )),
            confirmation_code: Set(confirmation_code.to_string()),
            ..Default::default()
        };

        let drp = drp.insert(self).await?;

        Ok(drp.try_convert()?)
    }
    async fn get_destination_registration_proposal_by_id(
        &self,
        id: String,
    ) -> Result<DestinationRegistrationProposal> {
        let drp =
            db_entities::destination_registration_proposal::Entity::find_by_id(id.parse::<i32>()?)
                .one(self)
                .await?
                .ok_or("could not find drp")?;
        Ok(drp.try_convert()?)
    }
    async fn create_user_destination(
        &self,
        request: &DestinationRegistrationProposal,
    ) -> Result<UserDestination> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let dest = db_entities::user_destination::ActiveModel {
            user_id: Set(request.user_id.clone()),
            destination: Set(hex::encode(
                request
                    .destination
                    .as_ref()
                    .ok_or("could not find destination")?
                    .encode_to_vec(),
            )),
            status: Set(user_destination::Status::Enabled as i32),
            created_at: Set(now.to_string()),
            updated_at: Set(now.to_string()),
            ..Default::default()
        };

        let dest = dest.insert(self).await?;

        Ok(dest.try_convert()?)
    }

    async fn get_user_destinations(
        &self,
        filter: UserDestinationFilter,
        page: Option<u64>,
    ) -> Result<Vec<UserDestination>> {
        let mut query = db_entities::user_destination::Entity::find()
            .filter(db_entities::user_destination::Column::DeletedAt.is_null());

        if let Some(id) = filter.id {
            query = query.filter(db_entities::user_destination::Column::Id.eq(id.parse::<i32>()?));
        }

        if let Some(user_id) = filter.user_id {
            query = query.filter(db_entities::user_destination::Column::UserId.eq(user_id));
        }

        let models;
        if let Some(page) = page {
            models = query.paginate(self, 20).fetch_page(page).await?;
        } else {
            models = query.all(self).await?;
        }

        Ok(models.try_convert()?)
    }

    async fn update_user_destination(&self, dest: UserDestination) -> Result<UserDestination> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let dest = db_entities::user_destination::ActiveModel {
            id: Set(dest.id.parse::<i32>()?),
            status: Set(dest.status),
            updated_at: Set(now.to_string()),
            ..Default::default()
        };

        let dest = dest.update(self).await?;

        Ok(dest.try_convert()?)
    }
    async fn delete_user_destination(&self, id: i32) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let dest = db_entities::user_destination::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(now.to_string())),
            ..Default::default()
        };

        dest.update(self).await?;

        Ok(())
    }
}
