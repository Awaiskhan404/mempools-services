use std::sync::Arc;

use cosmrs::{
    proto::cosmos::{
        base::abci::v1beta1::{AbciMessageLog, TxResponse},
        tx::v1beta1::Tx,
    },
    Any,
};
use dyn_clone::DynClone;
use mempools_api::api::{
    alert_notification_data::AlertNotificationData, destination::Destination, AlertSource,
    ConfirmDestinationRegistrationRequest, ConfirmDestinationRegistrationResponse, CosmosChainData,
    CreateAlertRequest, CreateChainRequest, CreateChainResponse, DestinationRegistrationProposal,
    EthChainData, GetChainsResponse, InitiateDestinationRegistrationRequest,
    InitiateDestinationRegistrationResponse, SubmitCognitoCodeGrantRequest,
    SubmitCognitoCodeGrantResponse, UpdateChainRequest, UpdateChainResponse, UserAlert,
    UserDestination,
};
use tokio::sync::RwLock;

use super::Result;

#[derive(Clone)]
pub struct RegisteryServices {
    pub filter_service: Box<dyn FilterServiceInterface>,
    pub alert_service: Box<dyn AlertServiceInterface>,
    pub notification_service: Box<dyn NotificationServiceInterface>,
    pub destination_service: Box<dyn DestinationServiceInterface>,
    pub chain_service: Box<dyn ChainServiceInterface>,
    pub auth_service: Box<dyn AuthServiceInterface>,
}

#[tonic::async_trait]
pub trait AuthServiceInterface: DynClone + Send + Sync + 'static {
    async fn submit_cognito_code_grant(
        &self,
        request: SubmitCognitoCodeGrantRequest,
    ) -> Result<SubmitCognitoCodeGrantResponse>;
}

#[tonic::async_trait]
pub trait FilterServiceInterface: DynClone + Send + Sync + 'static {
    async fn process_alert_source(&self, alert_source: ProcessAlertSourceRequeust) -> Result<()>;
    async fn process_alert_for_alert_source(
        &self,
        alert_source: ProcessAlertSourceRequeust,
        alert: UserAlert,
    ) -> Result<()>;
}

#[tonic::async_trait]
pub trait AlertServiceInterface: DynClone + Send + Sync + 'static {
    async fn get_alert_by_id(&self, alert_id: String) -> Result<UserAlert>;
    async fn get_alerts(&self, filter: AlertFilter, page: u64) -> Result<Vec<UserAlert>>;
    async fn create_alert(&self, req: &CreateAlertRequest) -> Result<UserAlert>;
    async fn update_alert(&self, alert: UserAlert) -> Result<UserAlert>;
    async fn delete_alert(&self, id: i32) -> Result<()>;
}

#[tonic::async_trait]
pub trait ChainServiceInterface: DynClone + Send + Sync + 'static {
    async fn get_chains(&self) -> Result<GetChainsResponse>;
    async fn create_chain(&self, req: &CreateChainRequest) -> Result<CreateChainResponse>;
    async fn update_chain(&self, req: &UpdateChainRequest) -> Result<UpdateChainResponse>;
}

#[tonic::async_trait]
pub trait NotificationServiceInterface: DynClone + Send + Sync + 'static {
    async fn get_notifications(
        &self,
        filter: NotificationFilter,
        page: u64,
    ) -> Result<Vec<mempools_api::api::AlertNotification>>;
    async fn send_notification(&self, notification: Notification) -> Result<()>;
}

#[tonic::async_trait]
pub trait DestinationServiceInterface: DynClone + Send + Sync + 'static {
    async fn initiate_destination_registration(
        &self,
        request: &InitiateDestinationRegistrationRequest,
    ) -> Result<InitiateDestinationRegistrationResponse>;
    async fn confirm_destination_registration(
        &self,
        request: &ConfirmDestinationRegistrationRequest,
    ) -> Result<ConfirmDestinationRegistrationResponse>;

    async fn get_user_destinations(
        &self,
        filter: UserDestinationFilter,
        page: Option<u64>,
    ) -> Result<Vec<UserDestination>>;

    async fn update_user_destination(&self, dest: UserDestination) -> Result<UserDestination>;

    async fn delete_user_destination(&self, id: i32) -> Result<()>;
    async fn get_destination_registration_proposal_by_id(
        &self,
        id: String,
    ) -> Result<DestinationRegistrationProposal>;
}

dyn_clone::clone_trait_object!(FilterServiceInterface);
dyn_clone::clone_trait_object!(AlertServiceInterface);
dyn_clone::clone_trait_object!(NotificationServiceInterface);
dyn_clone::clone_trait_object!(DestinationServiceInterface);
dyn_clone::clone_trait_object!(ChainServiceInterface);
dyn_clone::clone_trait_object!(AuthServiceInterface);

#[derive(Clone, Default)]
pub struct ServiceRegistery(Arc<RwLock<Option<RegisteryServices>>>);

impl ServiceRegistery {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }

    pub async fn register_services(&self, svcs: RegisteryServices) {
        *self.0.write().await = Some(svcs)
    }

    pub async fn get_services(&self) -> Result<RegisteryServices> {
        Ok(self
            .0
            .read()
            .await
            .as_ref()
            .ok_or("could not find registery")?
            .clone())
    }
}

#[derive(Clone, Default)]
pub struct AlertFilter {
    pub id: Option<i32>,
    pub user_id: Option<String>,
    pub chain_id: Option<i32>,
    pub alert_source: Option<AlertSource>,
}

#[derive(Clone, Default)]
pub struct UserDestinationFilter {
    pub id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Clone, Default)]
pub struct NotificationFilter {
    pub id: Option<i32>,
    pub alert_id: Option<i32>,
}

pub enum Notification {
    AlertNotification(AlertNotification),
    ConfirmationCodeNotification(ConfirmationCodeNotification),
}

pub struct AlertNotification {
    pub notification: AlertNotificationData,
    pub alert_id: String,
    pub alert_source_id: String,
}

pub struct ConfirmationCodeNotification {
    pub code: String,
    pub destination: Destination,
}

#[derive(Clone)]
pub enum ProcessAlertSourceRequeust {
    CosmosTx(Box<AlertSourceCosmosTx>),
    CosmosMsg(Box<AlertSourceCosmosMsg>),
    EthLog(Box<AlertSourceEthLog>),
    EthTx(Box<AlertSourceEthTx>),
}

#[derive(Clone)]
pub struct AlertSourceCosmosTx {
    pub chain_id: String,
    pub chain_data: CosmosChainData,
    pub tx: Tx,
    pub tx_resp: TxResponse,
    pub tx_hash: String,
}
#[derive(Clone)]
pub struct AlertSourceCosmosMsg {
    pub chain_id: String,
    pub chain_data: CosmosChainData,
    pub msg_log: Option<AbciMessageLog>,
    pub msg_index: u64,
    pub msg: Any,
    pub tx_hash: String,
}

#[derive(Clone)]
pub struct AlertSourceEthLog {
    pub chain_id: String,
    pub chain_data: EthChainData,
    pub tx_hash: String,
    pub log: web3::types::Log,
    pub log_index: u64,
}

#[derive(Clone)]
pub struct AlertSourceEthTx {
    pub chain_id: String,
    pub chain_data: EthChainData,
    pub tx_hash: String,
    pub tx: web3::types::Transaction,
    pub tx_resp: web3::types::TransactionReceipt,
}

#[derive(Clone)]
pub struct AlertSourceContext {
    pub id: String,
    pub chain_id: String,
    pub source_type: AlertSource,
}

impl ProcessAlertSourceRequeust {
    pub fn ctx(&self) -> AlertSourceContext {
        match self.clone() {
            ProcessAlertSourceRequeust::CosmosTx(tx) => AlertSourceContext {
                id: tx.tx_hash,
                chain_id: tx.chain_id,
                source_type: AlertSource::CosmosTx,
            },
            ProcessAlertSourceRequeust::CosmosMsg(msg) => AlertSourceContext {
                id: msg.tx_hash,
                chain_id: msg.chain_id,
                source_type: AlertSource::CosmosMsg,
            },
            ProcessAlertSourceRequeust::EthLog(log) => AlertSourceContext {
                id: log.tx_hash,
                chain_id: log.chain_id,
                source_type: AlertSource::EthLog,
            },
            ProcessAlertSourceRequeust::EthTx(tx) => AlertSourceContext {
                id: tx.tx_hash,
                chain_id: tx.chain_id,
                source_type: AlertSource::EthTx,
            },
        }
    }

    pub fn get_cosmos_msg(&self) -> Result<AlertSourceCosmosMsg> {
        if let Self::CosmosMsg(msg) = self {
            Ok(*msg.clone())
        } else {
            Err("alert source mistmatch".into())
        }
    }

    pub fn get_cosmos_tx(&self) -> Result<AlertSourceCosmosTx> {
        if let Self::CosmosTx(tx) = self {
            Ok(*tx.clone())
        } else {
            Err("alert source mistmatch".into())
        }
    }

    pub fn get_eth_log(&self) -> Result<AlertSourceEthLog> {
        if let Self::EthLog(log) = self {
            Ok(*log.clone())
        } else {
            Err("alert source mistmatch".into())
        }
    }

    pub fn get_eth_tx(&self) -> Result<AlertSourceEthTx> {
        if let Self::EthTx(tx) = self {
            Ok(*tx.clone())
        } else {
            Err("alert source mistmatch".into())
        }
    }
}
