use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationServiceConfig {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_address: String,
    pub telegram_bot_token: String,
    pub daily_notification_limit: u64,
}

impl Default for NotificationServiceConfig {
    fn default() -> Self {
        Self {
            smtp_username: "mempoolstest123@gmail.com".to_string(),
            smtp_password: "ycgkxwkdycqjizap".to_string(),
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_from_address: "mempoolstest123@gmail.com".to_string(),
            telegram_bot_token: "5863098705:AAFYUzVnHIE3I-1QHXbHBlV8Q4zcA0dSdr0".to_string(),
            daily_notification_limit: 200,
        }
    }
}
