use std::time::Duration;

use cosmrs::proto::cosmos::base::tendermint::v1beta1::{
    service_client::ServiceClient, GetLatestBlockRequest,
};
use cosmrs::proto::cosmos::{
    base::tendermint::v1beta1::service_client::ServiceClient as TendermintClient,
    tx::v1beta1::service_client::ServiceClient as TxClient,
};

use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use web3::{transports::Http, Web3};

use super::Result;

#[derive(Clone)]
pub struct CosmosClient {
    pub tendermint_client: TendermintClient<Channel>,
    pub tx_client: TxClient<Channel>,
}

impl CosmosClient {
    pub async fn new(mut url: String) -> Result<Self> {
        if let Some((_, urn)) = url.split_once("://") {
            url = urn.to_string();
        }

        if let Ok(client) = Self::new_with_security(url.clone(), true).await {
            Ok(client)
        } else {
            Self::new_with_security(url, false).await
        }
    }

    async fn new_with_security(mut url: String, secure: bool) -> Result<Self> {
        if secure {
            url = format!("https://{}", url);
        } else {
            url = format!("http://{}", url);
        }

        let mut endpoint = Endpoint::new(url)?
            .rate_limit(5, Duration::from_secs(1))
            .timeout(Duration::from_secs(7))
            .connect_timeout(Duration::from_secs(7));

        if secure {
            endpoint = endpoint.tls_config(ClientTlsConfig::new())?;
        }

        let channel = endpoint.connect().await?;
        let tendermint_client = ServiceClient::new(channel.clone());
        tendermint_client
            .clone()
            .get_latest_block(GetLatestBlockRequest {})
            .await?
            .get_ref()
            .clone()
            .block
            .ok_or("could not get block")?
            .header
            .ok_or("could not find block header")?;

        let tx_client = TxClient::new(channel.clone());

        Ok(CosmosClient {
            tendermint_client,
            tx_client,
        })
    }
}

pub async fn new_eth_client(url: &str) -> Result<Web3<Http>> {
    let transport = web3::transports::Http::new(url)?;
    let client = web3::Web3::new(transport);
    client.eth().block_number().await?;

    Ok(client)
}
