use mempools_api::api::{CreateAlertRequest, UserAlert};
use util::service_registry::{AlertFilter, AlertServiceInterface};

use util::Result;

use self::storage::AlertStorage;

pub mod storage;

#[derive(Clone)]
pub struct AlertService {
    store: Box<dyn AlertStorage>,
}

#[tonic::async_trait]
impl AlertServiceInterface for AlertService {
    async fn get_alert_by_id(&self, alert_id: String) -> Result<UserAlert> {
        let alert = self
            .get_alerts(
                AlertFilter {
                    id: Some(alert_id.parse::<i32>()?),
                    ..Default::default()
                },
                0,
            )
            .await?
            .first()
            .ok_or("alert not found")?
            .clone();

        Ok(alert)
    }

    async fn get_alerts(&self, filter: AlertFilter, page: u64) -> Result<Vec<UserAlert>> {
        self.store.get_alerts(filter, Some(page)).await
    }

    async fn create_alert(&self, req: &CreateAlertRequest) -> Result<UserAlert> {
        self.store.create_alert(req).await
    }

    async fn update_alert(&self, alert: UserAlert) -> Result<UserAlert> {
        self.store.update_alert(alert).await
    }
    async fn delete_alert(&self, id: i32) -> Result<()> {
        self.store.delete_alert(id).await
    }
}

impl AlertService {
    pub fn new<S: AlertStorage>(store: S) -> Self {
        Self {
            store: Box::new(store),
        }
    }
}
