use std::time::UNIX_EPOCH;

use cosmrs::proto::traits::Message;
use dyn_clone::DynClone;
use mempools_api::api::{AlertNotification, AlertNotificationData};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, Set,
};
use util::{convert::TryConvert, service_registry::NotificationFilter};

use util::Result;

#[tonic::async_trait]
pub trait NotificationStorage: DynClone + Send + Sync + 'static {
    async fn create_notification(
        &self,
        req: &super::AlertNotification,
    ) -> Result<AlertNotification>;
    async fn get_notifications(
        &self,
        filter: NotificationFilter,
        page: Option<u64>,
    ) -> Result<Vec<AlertNotification>>;

    async fn get_telegram_chat_id(&self, username: String) -> Result<String>;
    async fn set_telegram_chat_id(&self, username: String, chat_id: String) -> Result<()>;
}
dyn_clone::clone_trait_object!(NotificationStorage);

#[tonic::async_trait]
impl NotificationStorage for DatabaseConnection {
    async fn create_notification(
        &self,
        req: &super::AlertNotification,
    ) -> Result<AlertNotification> {
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos();

        let notification = db_entities::alert_notification::ActiveModel {
            notification_data: Set(hex::encode(
                AlertNotificationData {
                    alert_notification_data: Some(req.notification.clone()),
                }
                .encode_to_vec(),
            )),
            alert_id: Set(req.alert_id.parse::<i32>()?),
            alert_source_id: Set(req.alert_source_id.clone()),
            created_at: Set(now.to_string()),
            updated_at: Set(now.to_string()),
            ..Default::default()
        };

        let notification = notification.insert(self).await?;

        Ok(notification.try_convert()?)
    }

    async fn get_notifications(
        &self,
        filter: NotificationFilter,
        page: Option<u64>,
    ) -> Result<Vec<AlertNotification>> {
        let mut query = db_entities::alert_notification::Entity::find()
            .filter(db_entities::alert_notification::Column::DeletedAt.is_null());

        if let Some(id) = filter.id {
            query = query.filter(db_entities::alert_notification::Column::Id.eq(id));
        }

        if let Some(alert_id) = filter.alert_id {
            query = query.filter(db_entities::alert_notification::Column::AlertId.eq(alert_id));
        }

        let models;
        if let Some(page) = page {
            models = query.paginate(self, 20).fetch_page(page).await?;
        } else {
            models = query.all(self).await?;
        }

        Ok(models.try_convert()?)
    }

    async fn get_telegram_chat_id(&self, username: String) -> Result<String> {
        let row = db_entities::telegram_chat_id::Entity::find()
            .filter(db_entities::telegram_chat_id::Column::Username.eq(username))
            .one(self)
            .await?
            .ok_or("could not find chat id for username")?;

        Ok(row.chat_id)
    }

    async fn set_telegram_chat_id(&self, username: String, chat_id: String) -> Result<()> {
        if let Some(model) = db_entities::telegram_chat_id::Entity::find()
            .filter(db_entities::telegram_chat_id::Column::Username.eq(username.clone()))
            .one(self)
            .await?
        {
            let mut row = model.into_active_model();
            row.chat_id = Set(chat_id);
            row.update(self).await?;
        } else {
            let row = db_entities::telegram_chat_id::ActiveModel {
                username: Set(username.clone()),
                chat_id: Set(chat_id),
                ..Default::default()
            };
            row.insert(self).await?;
        };

        Ok(())
    }
}
