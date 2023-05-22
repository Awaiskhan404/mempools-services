use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("processed_blocks"))
                    .add_column(
                        ColumnDef::new(Alias::new("data"))
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // drop extra columns in future migration

        manager
            .rename_table(
                Table::rename()
                    .table(Alias::new("processed_blocks"), Alias::new("crawler"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
