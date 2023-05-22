use config::AuthServiceConfig;
use mempools_api::api::{SubmitCognitoCodeGrantRequest, SubmitCognitoCodeGrantResponse};

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, IssuerUrl, RedirectUrl, TokenResponse,
};
use util::{service_registry::AuthServiceInterface, Result};

pub mod config;

#[derive(Clone)]
pub struct AuthService {
    config: AuthServiceConfig,
}

#[tonic::async_trait]
impl AuthServiceInterface for AuthService {
    async fn submit_cognito_code_grant(
        &self,
        request: SubmitCognitoCodeGrantRequest,
    ) -> Result<SubmitCognitoCodeGrantResponse> {
        let cognito_config = self
            .config
            .cognito_config
            .as_ref()
            .ok_or("cognito config is not present")?;

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(format!(
                "https://cognito-idp.{}.amazonaws.com/{}",
                cognito_config.aws_region, cognito_config.cognito_user_pool_id
            ))?,
            async_http_client,
        )
        .await
        .map_err(|err| match err {
            openidconnect::DiscoveryError::Request(err) => match err {
                openidconnect::reqwest::Error::Reqwest(err) => err.to_string(),
                _ => err.to_string(),
            },
            _ => err.to_string(),
        })?;

        let oidc_client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(cognito_config.cognito_client_id.to_string()),
            Some(ClientSecret::new(
                cognito_config.cognito_client_secret.to_string(),
            )),
        )
        .set_redirect_uri(RedirectUrl::new(
            cognito_config.cognito_redirect_url.to_string(),
        )?);

        let token_response = oidc_client
            .exchange_code(AuthorizationCode::new(request.code.clone()))
            .request_async(async_http_client)
            .await?;

        Ok(SubmitCognitoCodeGrantResponse {
            id_token: token_response
                .id_token()
                .ok_or("id token not found in token response")?
                .to_string(),
        })
    }
}

impl AuthService {
    pub fn new(config: AuthServiceConfig) -> Self {
        Self { config }
    }
}
