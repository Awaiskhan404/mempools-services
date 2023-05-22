use mempools_api::api::{alert_notification_data::AlertNotificationData, UserAlert};

use util::{service_registry::ProcessAlertSourceRequeust, Result};

pub mod cosmos;
pub mod eth;

pub trait AlertSourceFilter: Send + Sync {
    fn filter(&self, alert_source: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData>;
}

impl TryInto<Box<dyn AlertSourceFilter>> for UserAlert {
    type Error = String;

    fn try_into(self) -> std::result::Result<Box<dyn AlertSourceFilter>, Self::Error> {
        let res: Box<dyn AlertSourceFilter> = match self
            .alert
            .ok_or("could not find alert in user_alert")?
            .chain_alert
            .ok_or("could not find chain_alert in alert")?
        {
            mempools_api::api::alert::ChainAlert::CosmosAlert(a) => {
                match a
                    .cosmos_alert
                    .ok_or("could not find cosmos_alert in chain_alert")?
                {
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosSendFunds(a) => Box::new(a),
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosMonitorFunds(a) => Box::new(a),
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosSmartContractEvents(a) => {
                        Box::new(a)
                    }
                    mempools_api::api::cosmos_alert::CosmosAlert::AlertCosmosTxOutcome(a) => Box::new(a),
                }
            }
            mempools_api::api::alert::ChainAlert::CosmosEvmAlert(a) => {
                match a
                    .cosmos_evm_alert
                    .ok_or("could not find cosmos_evm_alert in chain_alert")?
                {
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthSmartContractEvents(a) => {
                        Box::new(a)
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthMonitorFunds(a) => {
                        Box::new(a)
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertEthTxOutcome(a) => Box::new(a),
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertCosmosMonitorFunds(a) => {
                        Box::new(a)
                    }
                    mempools_api::api::cosmos_evm_alert::CosmosEvmAlert::AlertCosmosTxOutcome(a) => {
                        Box::new(a)
                    }
                }
            }
            mempools_api::api::alert::ChainAlert::EthAlert(a) => {
                match a
                    .eth_alert
                    .ok_or("could not find cosmos_evm_alert in chain_alert")?
                {
                    mempools_api::api::eth_alert::EthAlert::AlertEthSmartContractEvents(a) => {
                        Box::new(a)
                    }
                    mempools_api::api::eth_alert::EthAlert::AlertEthMonitorFunds(a) => Box::new(a),
                    mempools_api::api::eth_alert::EthAlert::AlertEthTxOutcome(a) => Box::new(a),
                }
            }
        };

        Ok(res)
    }
}
