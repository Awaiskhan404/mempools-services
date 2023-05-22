use std::env;

use auth_service::config::AuthServiceConfig;
use base64::Engine;
use notification_service::config::NotificationServiceConfig;
use serde::Deserialize;
use serde::Serialize;
use util::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub db_url: String,
    pub admins: Vec<String>,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            db_url: "sqlite::memory:".to_string(),
            admins: vec!["admin@admin.com".to_string()],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub application_config: ApplicationConfig,
    pub auth_config: AuthServiceConfig,
    pub notification_config: NotificationServiceConfig,
}

impl Config {
    pub fn from_env_or_default() -> Result<Self> {
        if let Ok(encoded_config) = env::var("CONFIG") {
            if encoded_config.is_empty() {
                return Err("config env var is empty".into());
            }
            let config_raw = base64::prelude::BASE64_STANDARD.decode(encoded_config)?;
            return Ok(serde_json::from_slice(&config_raw)?);
        }

        Ok(Config::default())
    }
}
