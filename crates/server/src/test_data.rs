use chain_service::storage::ChainStorage;
use db_migration::ToDbResult;
use destination_service::storage::DestinationStorage;
use mempools_api::api::{
    CosmosChainData, CosmosEvmChainData, CreateChainRequest, Destination, EthChainData,
    InitiateDestinationRegistrationRequest,
};
use sea_orm::DatabaseConnection;
use util::Result;

pub async fn add_test_data(db: &DatabaseConnection) -> Result<()> {
    let chains = vec![
        CreateChainRequest {
            name: "Juno mainnet".to_string(),
            icon: "".to_string(),
            chain_data: Some(mempools_api::api::ChainData {
                chain_data: Some(mempools_api::api::chain_data::ChainData::CosmosChainData(
                    CosmosChainData {
                        grpc_endpoint: "grpc-juno-ia.cosmosia.notional.ventures:443".to_string(),
                        bech32_prefix: "juno".to_string(),
                    },
                )),
            }),
        },
        CreateChainRequest {
            name: "Canto mainnet".to_string(),
            icon: "".to_string(),

            chain_data: Some(mempools_api::api::ChainData {
                chain_data: Some(
                    mempools_api::api::chain_data::ChainData::CosmosEvmChainData(
                        CosmosEvmChainData {
                            cosmos_chain_data: Some(CosmosChainData {
                                grpc_endpoint: "https://canto.gravitychain.io:9090".to_string(),
                                bech32_prefix: "canto".to_string(),
                            }),
                            eth_chain_data: Some(EthChainData {
                                eth_rpc_endpoint: "https://canto.gravitychain.io".to_string(),
                            }),
                        },
                    ),
                ),
            }),
        },
        CreateChainRequest {
            name: "Ethereum goerli testnet".to_string(),
            icon: "".to_string(),
            chain_data: Some(mempools_api::api::ChainData {
                chain_data: Some(mempools_api::api::chain_data::ChainData::EthChainData(
                    EthChainData {
                        eth_rpc_endpoint: "https://rpc.ankr.com/eth_goerli".to_string(),
                    },
                )),
            }),
        },
    ];

    for chain in chains {
        db.create_chain(&chain).await.to_db_result()?;
    }

    let drp = db
        .create_destination_registration_proposal(&InitiateDestinationRegistrationRequest {
            user_id: "1".to_string(),
            destination: Some(Destination {
                destination: Some(mempools_api::api::destination::Destination::Telegram(
                    "Shravan322".to_string(),
                )),
            }),
        })
        .await?;
    db.create_user_destination(&drp).await?;

    let drp = db
    .create_destination_registration_proposal(&InitiateDestinationRegistrationRequest {
        user_id: "1".to_string(),
        destination: Some(Destination {
            destination: Some(mempools_api::api::destination::Destination::DiscordWebhook(
                "https://discord.com/api/webhooks/1101475611656069231/IslBUMec8xav5EWDUqN64E2HG398xjiN2at2oXN0I9TTuLWG7Vf9QqdboteVIdYktcLR".to_string(),
            )),
        }),
    })
    .await?;
    db.create_user_destination(&drp).await?;

    Ok(())
}
