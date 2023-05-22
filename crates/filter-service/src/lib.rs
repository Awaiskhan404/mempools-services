use log::{info, warn};
use mempools_api::api::{user_alert::Status, UserAlert};

use util::{
    service_registry::{
        AlertFilter, AlertNotification, FilterServiceInterface, Notification,
        ProcessAlertSourceRequeust, ServiceRegistery,
    },
    Result,
};

use alerts::AlertSourceFilter;

#[derive(Clone)]
pub struct FilterService {
    registery: ServiceRegistery,
}

impl FilterService {
    pub fn new(registery: ServiceRegistery) -> Result<Self> {
        Ok(Self { registery })
    }
}

#[tonic::async_trait]
impl FilterServiceInterface for FilterService {
    async fn process_alert_source(&self, alert_source: ProcessAlertSourceRequeust) -> Result<()> {
        let registery = self.registery.get_services().await?;
        let alert_service = registery.alert_service;

        let mut page = 0;
        loop {
            let ctx = alert_source.ctx();
            let filter = AlertFilter {
                alert_source: Some(ctx.source_type),
                chain_id: Some(ctx.chain_id.parse::<i32>()?),
                ..Default::default()
            };
            let alerts = alert_service.get_alerts(filter, page).await?;

            if alerts.is_empty() {
                break;
            }

            for alert in alerts {
                let ctx = ctx.clone();
                let svc = registery.filter_service.clone();
                let alert_source = alert_source.clone();
                let alert_source_id = ctx.id;
                let alert_source_type_name = ctx.source_type.as_str_name();
                tokio::spawn(async move {
                    if let Err(err) = svc
                        .process_alert_for_alert_source(alert_source, alert.clone())
                        .await
                    {
                        warn!(
                            "failed to filter alert source {} {}, for alert id {},reason - {}",
                            alert_source_type_name, alert_source_id, alert.id, err
                        )
                    };
                });
            }

            page += 1;
        }

        Ok(())
    }

    async fn process_alert_for_alert_source(
        &self,
        alert_source: ProcessAlertSourceRequeust,
        alert: UserAlert,
    ) -> Result<()> {
        let ctx = alert_source.ctx();
        let registery = self.registery.get_services().await?;

        if alert.status == Status::Disabled as i32 {
            return Ok(());
        }

        info!(
            "processing alert source {} {} for alert {} in chain {}",
            ctx.source_type.as_str_name(),
            ctx.id,
            alert.id,
            ctx.chain_id
        );

        let tx_alert: Box<dyn AlertSourceFilter> = alert.clone().try_into()?;
        let notification = if let Ok(notification) = tx_alert.filter(&alert_source) {
            notification
        } else {
            return Ok(());
        };

        info!(
            "sending notification for alert source {} {} on alert {}",
            ctx.source_type.as_str_name(),
            ctx.id,
            alert.id
        );
        registery
            .notification_service
            .send_notification(Notification::AlertNotification(AlertNotification {
                notification,
                alert_id: alert.id.clone(),
                alert_source_id: ctx.id,
            }))
            .await?;

        Ok(())
    }
}
