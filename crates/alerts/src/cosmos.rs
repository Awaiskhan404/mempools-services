use std::collections::HashMap;

use cosmrs::tx::Msg;
use cw20_base::msg::ExecuteMsg;

use mempools_api::api::{
    alert_cosmos_tx_outcome::CosmosTxOutcome, alert_notification_data::AlertNotificationData,
    monitor_funds_coin_notification_data::CoinAmount, AlertCosmosMonitorFunds,
    AlertCosmosSendFunds, AlertCosmosSmartContractEvents, AlertCosmosTxOutcome,
    MonitorFundsCoinNotificationData, MonitorFundsTokenNotificationData,
    SmartContractEventsNotificationData, TxOutcomeNotificationData,
};

use util::{get_signers_from_tx, service_registry::ProcessAlertSourceRequeust, Result};

use crate::AlertSourceFilter;

impl AlertSourceFilter for AlertCosmosTxOutcome {
    fn filter(&self, alert_source: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = alert_source.get_cosmos_tx()?;

        if !self.signer.is_empty() {
            let signers =
                get_signers_from_tx(ctx.chain_data.bech32_prefix, cosmrs::Tx::try_from(ctx.tx)?);
            if !signers.contains(&self.signer) {
                return Err("signer not found".into());
            }
        }

        let matched =
            match CosmosTxOutcome::from_i32(self.outcome).ok_or("invalid i32 val for enum")? {
                CosmosTxOutcome::Succeeded => ctx.tx_resp.code == 0,
                CosmosTxOutcome::Failed => ctx.tx_resp.code != 0,
            };

        if !matched {
            return Err("tx outcome mismatch".into());
        }

        Ok(AlertNotificationData::TxOutcome(
            TxOutcomeNotificationData {
                signer: self.signer.clone(),
                outcome: self.outcome().as_str_name().to_string(),
                tx_hash: alert_source.get_cosmos_tx()?.tx_hash,
            },
        ))
    }
}

impl AlertSourceFilter for AlertCosmosMonitorFunds {
    fn filter(&self, req: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = req.get_cosmos_msg()?;
        let msg = &ctx.msg;

        if let Ok(bank_send) = cosmrs::bank::MsgSend::from_any(msg) {
            let notification =
                AlertNotificationData::MonitorFundsCoin(MonitorFundsCoinNotificationData {
                    from: bank_send.from_address.to_string(),
                    to: bank_send.to_address.to_string(),
                    amount: bank_send
                        .amount
                        .iter()
                        .map(|c| CoinAmount {
                            amount: c.amount.to_string(),
                            denom: c.denom.to_string(),
                        })
                        .collect(),
                    tx_hash: ctx.tx_hash.clone(),
                });

            if self.address.is_empty() {
                return Ok(notification);
            }

            if self.address == bank_send.from_address.to_string()
                || self.address == bank_send.to_address.to_string()
            {
                return Ok(notification);
            }
        }

        if let Ok(exec_contract) = cosmrs::cosmwasm::MsgExecuteContract::from_any(msg) {
            let cw_20_exec_msg: ExecuteMsg = serde_json::from_slice(&exec_contract.msg)?;
            if let ExecuteMsg::Transfer { recipient, amount } = cw_20_exec_msg {
                let notification =
                    AlertNotificationData::MonitorFundsToken(MonitorFundsTokenNotificationData {
                        from: exec_contract.sender.to_string(),
                        to: recipient.clone(),
                        amount: amount.to_string(),
                        tx_hash: ctx.tx_hash.clone(),
                        contract_addr: exec_contract.contract.to_string(),
                    });

                if self.address.is_empty() {
                    return Ok(notification);
                }

                if self.address == exec_contract.sender.to_string() || self.address == recipient {
                    return Ok(notification);
                }
            }
        }

        Err("address not related to message".into())
    }
}

impl AlertSourceFilter for AlertCosmosSendFunds {
    fn filter(&self, req: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = req.get_cosmos_msg()?;
        let msg = &ctx.msg;
        let tx_hash = ctx.tx_hash;

        if let Ok(bank_send) = cosmrs::bank::MsgSend::from_any(msg) {
            if !self.from.is_empty() && bank_send.from_address.to_string() != self.from {
                return Err(format!(
                    "filter_send_funds failed, wrong from address - expected - {}, got - {}",
                    bank_send.from_address, self.from
                )
                .into());
            }

            if !self.to.is_empty() && bank_send.to_address.to_string() != self.to {
                return Err(format!(
                    "filter_send_funds failed, wrong to address - expected - {}, got - {}",
                    bank_send.to_address, self.to
                )
                .into());
            }

            let notification =
                AlertNotificationData::MonitorFundsCoin(MonitorFundsCoinNotificationData {
                    from: bank_send.from_address.to_string(),
                    to: bank_send.to_address.to_string(),
                    amount: bank_send
                        .amount
                        .iter()
                        .map(|c| CoinAmount {
                            amount: c.amount.to_string(),
                            denom: c.denom.to_string(),
                        })
                        .collect(),
                    tx_hash,
                });

            return Ok(notification);
        }

        if let Ok(exec_contract) = cosmrs::cosmwasm::MsgExecuteContract::from_any(msg) {
            let cw_20_exec_msg: ExecuteMsg = serde_json::from_slice(&exec_contract.msg)?;
            if let ExecuteMsg::Transfer { recipient, amount } = cw_20_exec_msg {
                if !self.from.is_empty() && exec_contract.sender.to_string() != self.from {
                    return Err(format!(
                        "filter_send_funds failed, wrong from address - expected - {}, got - {}",
                        exec_contract.sender, self.from
                    )
                    .into());
                }

                if !self.to.is_empty() && recipient != self.to {
                    return Err(format!(
                        "filter_send_funds failed, wrong to address - expected - {}, got - {}",
                        recipient, self.to
                    )
                    .into());
                }

                let notification =
                    AlertNotificationData::MonitorFundsToken(MonitorFundsTokenNotificationData {
                        from: exec_contract.sender.to_string(),
                        to: recipient,
                        amount: amount.to_string(),
                        tx_hash,
                        contract_addr: exec_contract.contract.to_string(),
                    });

                return Ok(notification);
            }
        }

        Err("incorrect message type".into())
    }
}

impl AlertSourceFilter for AlertCosmosSmartContractEvents {
    fn filter(&self, req: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = req.get_cosmos_msg()?;
        let msg = &ctx.msg;

        if let Ok(exec_contract) = cosmrs::cosmwasm::MsgExecuteContract::from_any(msg) {
            let got_contract_addr = exec_contract.contract.to_string();
            if !self.address.is_empty() && got_contract_addr != self.address {
                return Err("msg not from correct contract".into());
            }

            let mut events = HashMap::new();
            for e in &ctx.msg_log.ok_or("could not find msg log")?.events {
                let mut attrs = HashMap::new();
                for attr in &e.attributes {
                    attrs.insert(attr.key.clone(), attr.value.clone());
                }

                events.insert(e.r#type.clone(), attrs);
            }

            let attrs = events
                .get(&self.event_name)
                .ok_or("could not find event in msg")?
                .clone();

            for (k, v) in &self.event_attributes {
                if attrs.get(k) != Some(v) {
                    return Err("missing event in contract execution".into());
                }
            }

            return Ok(AlertNotificationData::ScEvents(
                SmartContractEventsNotificationData {
                    contract_addr: got_contract_addr,
                    event_name: self.event_name.clone(),
                    event_attributes: self.event_attributes.clone(),
                    tx_hash: ctx.tx_hash,
                },
            ));
        }

        Err("msg not related to contract".into())
    }
}
