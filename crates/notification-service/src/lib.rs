use std::{collections::HashMap, sync::Arc, time::Duration};

use config::NotificationServiceConfig;
// use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use log::{info, warn};
use mempools_api::api::destination::Destination;
use mempools_api::api::{destination, user_destination};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use telegram_bot::{ChatId, ChatRef, MessageKind, SendMessage, UpdateKind};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use util::{
    convert::TryConvert,
    get_sha256_hash,
    service_registry::{ServiceRegistery, UserDestinationFilter},
};
use util::{
    service_registry::{
        AlertNotification, ConfirmationCodeNotification, Notification, NotificationFilter,
        NotificationServiceInterface,
    },
    Result,
};

use crate::notification::AlertNotificationContentImplementation;

use self::{
    notification::{
        ConfirmationCodeNotificationContent, NotificationContent, RateLimitNotification,
    },
    storage::NotificationStorage,
};

pub mod config;
pub mod notification;
pub mod storage;

#[derive(Clone)]
pub struct NotificationService {
    config: NotificationServiceConfig,
    registery: ServiceRegistery,
    storage: Box<dyn NotificationStorage>,
    http_client: reqwest::Client,
    // smtp_client: SmtpTransport,
    telegram_bot_client: telegram_bot::Api,
    counter: Arc<RwLock<HashMap<String, u64>>>,
}

#[tonic::async_trait]
impl NotificationServiceInterface for NotificationService {
    async fn send_notification(&self, notification: Notification) -> Result<()> {
        match notification {
            Notification::AlertNotification(n) => self.send_alert_notification(n).await?,
            Notification::ConfirmationCodeNotification(n) => {
                self.send_confirmation_code_notification(n).await?
            }
        }

        Ok(())
    }

    async fn get_notifications(
        &self,
        filter: NotificationFilter,
        page: u64,
    ) -> Result<Vec<mempools_api::api::AlertNotification>> {
        self.storage.get_notifications(filter, Some(page)).await
    }
}

impl NotificationService {
    pub fn new<S: NotificationStorage>(
        config: NotificationServiceConfig,
        registery: ServiceRegistery,
        storage: S,
    ) -> Result<Self> {
        // let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        // let smtp_client = SmtpTransport::relay(&config.smtp_host)?
        //     .credentials(creds)
        //     .build();
        let telegram_bot_client = telegram_bot::Api::new(config.telegram_bot_token.clone());
        let counter = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            registery,
            storage: Box::new(storage),
            // smtp_client,
            http_client: Client::new(),
            telegram_bot_client,
            counter,
        })
    }

    async fn send_alert_notification(&self, alert_notification: AlertNotification) -> Result<()> {
        self.storage
            .create_notification(&alert_notification)
            .await?;

        let registery = self.registery.get_services().await?;
        let alert = registery
            .alert_service
            .get_alert_by_id(alert_notification.alert_id.clone())
            .await?;

        let mut destinations = vec![];
        for dest_id in alert.destination_ids.clone() {
            destinations.push(
                registery
                    .destination_service
                    .get_user_destinations(
                        UserDestinationFilter {
                            id: Some(dest_id),
                            ..Default::default()
                        },
                        None,
                    )
                    .await?
                    .first()
                    .ok_or("could not find user destination")?
                    .clone(),
            )
        }

        for dst in destinations {
            if dst.status == user_destination::Status::Enabled as i32 {
                let notification = Box::new(AlertNotificationContentImplementation(
                    alert_notification.notification.try_convert()?,
                    alert.clone(),
                ));
                let destination = &dst
                    .destination
                    .ok_or("could not find destination")?
                    .destination
                    .ok_or("could not find destination")?;

                self.send_notification_to_destination(notification, destination.clone())
                    .await?;
            }
        }

        info!(
            "sent notification for alert source {} {} on alert {}",
            alert.alert_source().as_str_name(),
            alert_notification.alert_source_id,
            alert_notification.alert_id
        );

        Ok(())
    }

    async fn send_confirmation_code_notification(
        &self,
        notification: ConfirmationCodeNotification,
    ) -> Result<()> {
        self.send_notification_to_destination(
            Box::new(ConfirmationCodeNotificationContent(
                notification.destination.get_identifier(),
                notification.code,
            )),
            notification.destination,
        )
        .await?;

        Ok(())
    }

    async fn send_notification_to_destination(
        &self,
        mut notification: Box<dyn NotificationContent>,
        to: Destination,
    ) -> Result<()> {
        let mut counter = self.counter.write().await;
        let notification_count = *counter.get(&to.get_identifier()).unwrap_or(&0);

        if notification_count > self.config.daily_notification_limit {
            return Err(format!("notification limit reached for {}", to.get_identifier()).into());
        }

        counter.insert(to.get_identifier(), notification_count + 1);

        drop(counter);

        if notification_count == self.config.daily_notification_limit {
            notification = Box::new(RateLimitNotification);
        }

        match to {
            destination::Destination::Email(_email) => {
                // let email = Message::builder()
                //     .from(Mailbox {
                //         name: None,
                //         email: Address::from_str(self.config.smtp_from_address.as_str())?,
                //     })
                //     .to(Mailbox {
                //         name: None,
                //         email: Address::from_str(email)?,
                //     })
                //     .subject(subject)
                //     .header(ContentType::TEXT_PLAIN)
                //     .body(message)?;

                // self.smtp_client.send(&email)?;
            }
            destination::Destination::Webhook(host) => {
                let resp = self
                    .http_client
                    .post(host.clone())
                    .json(&WebhookNotificationMessage {
                        notification: notification.webhook(),
                    })
                    .send()
                    .await?;
                if resp.status() != 200 {
                    warn!(
                        "failed to send registration code to webhook at {} - recieved response {:?}",
                        host, resp
                    )
                }
            }
            destination::Destination::Telegram(username) => {
                let chat_id = self.storage.get_telegram_chat_id(username.clone()).await?;
                self.telegram_bot_client
                    .send_timeout(
                        SendMessage::new(
                            ChatRef::from_chat_id(ChatId::new(chat_id.parse()?)),
                            notification.telegram(),
                        ),
                        Duration::from_secs(5),
                    )
                    .await?;
            }
            destination::Destination::DiscordWebhook(uri) => {
                // You don't need a token when you are only dealing with webhooks.
                let http = serenity::http::Http::new("");
                let webhook = serenity::model::webhook::Webhook::from_url(&http, &uri).await?;
                webhook
                    .execute(&http, false, |w| {
                        w.content(notification.telegram()).username("Mempools")
                    })
                    .await?;
            }
        }

        Ok(())
    }

    pub fn spawn_daemons(&self) {
        let svc = self.clone();
        tokio::task::spawn(async move { svc.listen_on_telegram().await });

        let svc = self.clone();
        tokio::task::spawn(async move {
            svc.reset_counter_by_period(Duration::from_secs(3600 * 24))
                .await
        });
    }

    pub async fn reset_counter_by_period(&self, duration: Duration) {
        loop {
            tokio::time::sleep(duration).await;
            self.counter.write().await.clear();
            info!("cleared notification counter")
        }
    }

    pub async fn listen_on_telegram(&self) {
        loop {
            if let Err(err) = self.try_listen_on_telegram().await {
                warn!(
                    "telegram listener crashed! restarting in 5 seconds - {}",
                    err
                )
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    async fn try_listen_on_telegram(&self) -> Result<()> {
        let mut stream = self.telegram_bot_client.stream();
        while let Some(update) = stream.next().await {
            if let UpdateKind::Message(message) = update?.kind {
                if let Some(username) = message.from.username.clone() {
                    if self
                        .storage
                        .get_telegram_chat_id(username.clone())
                        .await
                        .is_err()
                    {
                        self.storage
                            .set_telegram_chat_id(username.clone(), message.chat.id().to_string())
                            .await?;
                        info!("chat_id was set for username {}", username)
                    }

                    if let MessageKind::Text { data, .. } = message.kind {
                        if data.starts_with("/start") {
                            self.telegram_bot_client
                                .send_timeout(
                                    SendMessage::new(message.chat.id(),
                                    format!("Thank you for connecting with the mempools telegram bot. Please navigate back to the application and input your telegram username '{}' to continue (username is case sensitive).",username)),
                                    Duration::from_secs(5),
                                )
                                .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct WebhookNotificationMessage {
    pub notification: String,
}

pub trait Identifier {
    fn get_identifier(&self) -> String;
}

impl Identifier for Destination {
    fn get_identifier(&self) -> String {
        match self {
            Destination::Email(email) => format!("email-{}", email),
            Destination::Webhook(webhook) => format!("webhook-{}", webhook),
            Destination::Telegram(telegram_username) => format!("telegram-{}", telegram_username),
            Destination::DiscordWebhook(uri) => {
                format!("discord-webhook-{}", get_sha256_hash(uri.as_bytes()))
            }
        }
    }
}
