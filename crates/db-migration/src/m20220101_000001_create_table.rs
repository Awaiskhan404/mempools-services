use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Alert {
    Table,

    Id,
    UserId,
    ChainId,
    DestinationIds,
    Filter,
    Message,
    Status,
}

#[derive(Iden)]
enum Chain {
    Table,

    Id,
    Name,
    Icon,
    GRPCEndpoint,
    Status,
}

#[derive(Iden)]
enum ProcessedBlocks {
    Table,

    Id,
    ChainId,
    LastProcessedBlock,
}

#[derive(Iden)]
enum UserDestination {
    Table,

    Id,
    UserId,
    Destination,
}

#[derive(Iden)]
enum AlertNotification {
    Table,

    Id,
    Message,
    AlertId,
    TxHash,
}

#[derive(Iden)]
enum TelegramChatId {
    Table,

    Id,
    Username,
    ChatId,
}

#[derive(Iden)]
enum DestinationRegistrationProposal {
    Table,

    Id,
    UserId,
    ConfirmationCode,
    Destination,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chain::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chain::Name).string().not_null())
                    .col(ColumnDef::new(Chain::Icon).string().not_null())
                    .col(ColumnDef::new(Chain::GRPCEndpoint).string().not_null())
                    .col(ColumnDef::new(Chain::Status).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Alert::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alert::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alert::UserId).string().not_null())
                    .col(ColumnDef::new(Alert::ChainId).integer().not_null())
                    .col(ColumnDef::new(Alert::DestinationIds).string().not_null())
                    .col(ColumnDef::new(Alert::Filter).string().not_null())
                    .col(ColumnDef::new(Alert::Message).string().not_null())
                    .col(ColumnDef::new(Alert::Status).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alert_chain")
                            .from(Alert::Table, Alert::ChainId)
                            .to(Chain::Table, Chain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProcessedBlocks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProcessedBlocks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessedBlocks::ChainId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessedBlocks::LastProcessedBlock)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_processed_blocks_chain")
                            .from(ProcessedBlocks::Table, ProcessedBlocks::ChainId)
                            .to(Chain::Table, Chain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DestinationRegistrationProposal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DestinationRegistrationProposal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DestinationRegistrationProposal::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DestinationRegistrationProposal::ConfirmationCode)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DestinationRegistrationProposal::Destination)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserDestination::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserDestination::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserDestination::UserId).string().not_null())
                    .col(
                        ColumnDef::new(UserDestination::Destination)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AlertNotification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlertNotification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AlertNotification::Message)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlertNotification::AlertId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlertNotification::TxHash)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alert_notification_alert")
                            .from(AlertNotification::Table, AlertNotification::AlertId)
                            .to(Alert::Table, Alert::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TelegramChatId::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TelegramChatId::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TelegramChatId::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TelegramChatId::ChatId).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_alert_user")
                    .table(Alert::Table)
                    .col(Alert::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_alert_chain")
                    .table(Alert::Table)
                    .col(Alert::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_pb_chain")
                    .table(ProcessedBlocks::Table)
                    .col(ProcessedBlocks::ChainId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_dest_user")
                    .table(UserDestination::Table)
                    .col(UserDestination::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_alert_noti_alert")
                    .table(AlertNotification::Table)
                    .col(AlertNotification::AlertId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_telechatId_username")
                    .table(TelegramChatId::Table)
                    .col(TelegramChatId::Username)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .name("IX_drp_user")
                    .table(DestinationRegistrationProposal::Table)
                    .col(DestinationRegistrationProposal::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
