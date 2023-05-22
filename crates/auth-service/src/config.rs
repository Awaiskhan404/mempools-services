use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AuthServiceConfig {
    pub cognito_config: Option<CognitoConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CognitoConfig {
    pub cognito_client_id: String,
    pub cognito_client_secret: String,
    pub cognito_redirect_url: String,
    pub cognito_user_pool_id: String,
    pub aws_region: String,
}
