use sea_orm_migration::{prelude::*, sea_orm::EntityTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert_notification"))
                    .rename_column(Alias::new("message"), Alias::new("notification_data"))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        db_entities::alert_notification::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}
