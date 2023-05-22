use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert"))
                    .rename_column(Alias::new("filter"), Alias::new("alert"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert"))
                    .add_column(
                        ColumnDef::new(Alias::new("alert_source"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert"))
                    .add_column(
                        ColumnDef::new(Alias::new("name"))
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_alert_alert_source")
                    .table(Alias::new("alert"))
                    .col(Alias::new("alert_source"))
                    .to_owned(),
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(Alias::new("alert"), Alias::new("user_alert"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("alert_notification"))
                    .rename_column(Alias::new("tx_hash"), Alias::new("alert_source_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("chain"))
                    .add_column(
                        ColumnDef::new(Alias::new("bech32_prefix"))
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
