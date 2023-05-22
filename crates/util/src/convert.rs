use cosmrs::proto::traits::Message;
use mempools_api::api::{
    Alert, AlertNotification, AlertNotificationData, AlertSource, Chain,
    DestinationRegistrationProposal, UserAlert, UserDestination,
};

use super::Result;

pub trait TryConvert<T> {
    fn try_convert(&self) -> Result<T>;
}

impl<T, M: TryConvert<T>> TryConvert<Vec<T>> for Vec<M> {
    fn try_convert(&self) -> Result<Vec<T>> {
        let mut views = vec![];
        for model in self {
            let view = model.try_convert()?;
            views.push(view);
        }

        Ok(views)
    }
}

impl TryConvert<UserAlert> for db_entities::user_alert::Model {
    fn try_convert(&self) -> Result<UserAlert> {
        Ok(UserAlert {
            id: self.id.to_string(),
            user_id: self.user_id.clone(),
            alert: Some(Alert::decode(hex::decode(self.alert.clone())?.as_slice())?),
            message: self.message.clone(),
            destination_ids: self
                .destination_ids
                .split(',')
                .map(|v| v.to_string())
                .collect(),
            status: self.status,
            chain_id: self.chain_id.to_string(),
            alert_source: self.alert_source,
            name: self.name.clone(),
        })
    }
}

impl TryConvert<UserDestination> for db_entities::user_destination::Model {
    fn try_convert(&self) -> Result<UserDestination> {
        let dest = self.clone();
        Ok(UserDestination {
            id: dest.id.to_string(),
            user_id: dest.user_id,
            destination: Some(mempools_api::api::Destination::decode(
                hex::decode(dest.destination)?.as_slice(),
            )?),
            status: self.status,
        })
    }
}

impl TryConvert<DestinationRegistrationProposal>
    for db_entities::destination_registration_proposal::Model
{
    fn try_convert(&self) -> Result<DestinationRegistrationProposal> {
        let drp = self.clone();
        Ok(DestinationRegistrationProposal {
            id: drp.id.to_string(),
            user_id: drp.user_id,
            confirmation_code: drp.confirmation_code,
            destination: Some(mempools_api::api::Destination::decode(
                hex::decode(drp.destination)?.as_slice(),
            )?),
        })
    }
}

impl TryConvert<i32> for Alert {
    fn try_convert(&self) -> Result<i32> {
        let alert_source = match self
            .chain_alert
            .as_ref()
            .ok_or("could not find chain_alert in alert")?
        {
            mempools_api::api::alert::ChainAlert::CosmosAlert(a) => {
                match a
                    .cosmos_alert
                    .as_ref()
                    .ok_or("could not find cosmos_alert in chain_alert")?
                {
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosSendFunds(_) => {
                        AlertSource::CosmosMsg
                    }
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosMonitorFunds(_) => {
                        AlertSource::CosmosMsg
                    }
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosSmartContractEvents(_) => {
                        AlertSource::CosmosMsg
                    }
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosTxOutcome(_) => {
                        AlertSource::CosmosTx
                    }
                }
            }
            mempools_api::api::alert::ChainAlert::CosmosEvmAlert(a) => {
                match a
                    .cosmos_evm_alert
                    .as_ref()
                    .ok_or("could not find cosmos_evm_alert in chain_alert")?
                {
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthSmartContractEvents(_) => {
                        AlertSource::EthLog
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthMonitorFunds(_) => {
                        AlertSource::EthTx
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthTxOutcome(_) => {
                        AlertSource::EthTx
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertCosmosMonitorFunds(_) => {
                        AlertSource::CosmosMsg
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertCosmosTxOutcome(_) => {
                        AlertSource::CosmosTx
                    }
                }
            }
            mempools_api::api::alert::ChainAlert::EthAlert(a) => {
                match a
                    .eth_alert
                    .as_ref()
                    .ok_or("could not find cosmos_evm_alert in chain_alert")?
                {
                    mempools_api::api::eth_alert::EthAlert::AlertEthSmartContractEvents(_) => {
                        AlertSource::EthLog
                    }
                    mempools_api::api::eth_alert::EthAlert::AlertEthMonitorFunds(_) => {
                        AlertSource::EthTx
                    }
                    mempools_api::api::eth_alert::EthAlert::AlertEthTxOutcome(_) => {
                        AlertSource::EthTx
                    }
                }
            }
        };

        Ok(alert_source as i32)
    }
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl<T, M: Convert<T>> Convert<Vec<T>> for Vec<M> {
    fn convert(self) -> Vec<T> {
        let mut views = vec![];
        for model in self {
            let view = model.convert();
            views.push(view);
        }

        views
    }
}

impl TryConvert<Chain> for db_entities::chain::Model {
    fn try_convert(&self) -> Result<Chain> {
        let chain = self.clone();
        let chain = Chain {
            id: chain.id.to_string(),
            name: chain.name,
            icon: chain.icon,
            status: chain.status,
            chain_data: Some(mempools_api::api::ChainData::decode(
                hex::decode(chain.chain_data)?.as_slice(),
            )?),
        };
        Ok(chain)
    }
}

impl TryConvert<AlertNotification> for db_entities::alert_notification::Model {
    fn try_convert(&self) -> Result<AlertNotification> {
        let notification = self.clone();
        Ok(AlertNotification {
            id: notification.id.to_string(),
            notification_data: Some(AlertNotificationData::decode(
                hex::decode(notification.notification_data)?.as_slice(),
            )?),
            alert_id: notification.alert_id.to_string(),
            alert_source_id: notification.alert_source_id,
        })
    }
}
