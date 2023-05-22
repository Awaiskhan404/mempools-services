use std::time::UNIX_EPOCH;

use mempools_api::api::user_alert;
use sea_orm_migration::prelude::*;

use crate::ToDbResult;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // drop uneeded columns

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .drop_column(Alias::new("bech32_prefix"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .drop_column(Alias::new("grpc_endpoint"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("crawler"))
                    .drop_column(Alias::new("last_processed_block"))
                    .to_owned(),
            )
            .await?;

        // actual migrations
        let now = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .to_db_result()?
            .as_nanos();

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_destination"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_destination"))
                    .add_column(
                        ColumnDef::new(Alias::new("updated_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_destination"))
                    .add_column(ColumnDef::new(Alias::new("deleted_at")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_alert"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_alert"))
                    .add_column(
                        ColumnDef::new(Alias::new("updated_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_alert"))
                    .add_column(ColumnDef::new(Alias::new("deleted_at")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .add_column(
                        ColumnDef::new(Alias::new("updated_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .add_column(ColumnDef::new(Alias::new("deleted_at")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert_notification"))
                    .add_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert_notification"))
                    .add_column(
                        ColumnDef::new(Alias::new("updated_at"))
                            .string()
                            .not_null()
                            .default(now.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert_notification"))
                    .add_column(ColumnDef::new(Alias::new("deleted_at")).string().null())
                    .to_owned(),
            )
            .await?;

        // add status to user destinations

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_destination"))
                    .add_column(
                        ColumnDef::new(Alias::new("status"))
                            .integer()
                            .not_null()
                            .default(user_alert::Status::Enabled as i32),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
