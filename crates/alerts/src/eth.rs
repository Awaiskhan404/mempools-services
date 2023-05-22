use std::collections::HashMap;

use base64::Engine;

use mempools_api::api::{
    alert_eth_tx_outcome::EthTxOutcome, alert_notification_data::AlertNotificationData,
    monitor_funds_coin_notification_data::CoinAmount, AlertEthMonitorFunds,
    AlertEthSmartContractEvents, AlertEthTxOutcome, MonitorFundsCoinNotificationData,
    MonitorFundsTokenNotificationData, SmartContractEventsNotificationData,
    TxOutcomeNotificationData,
};

use web3::ethabi::RawLog;

use util::HashString;
use util::{service_registry::ProcessAlertSourceRequeust, Result};

use crate::AlertSourceFilter;

const ERC20_ABI: &str = r#"[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_spender","type":"address"},{"name":"_value","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"balance","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"},{"name":"_spender","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"payable":true,"stateMutability":"payable","type":"fallback"},{"anonymous":false,"inputs":[{"indexed":true,"name":"owner","type":"address"},{"indexed":true,"name":"spender","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"from","type":"address"},{"indexed":true,"name":"to","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Transfer","type":"event"}]"#;

impl AlertSourceFilter for AlertEthSmartContractEvents {
    fn filter(&self, alert_source: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = alert_source.get_eth_log()?;
        let contract_addr = self.contract_addr.to_ascii_lowercase();
        let got_contract_addr = ctx.log.address.hash_string()?;
        if !contract_addr.is_empty() && contract_addr != got_contract_addr {
            return Err("log does not belong to contract".into());
        }

        let topics = ctx.log.topics;
        let data = ctx.log.data;

        let abi = web3::ethabi::Contract::load(
            base64::prelude::BASE64_STANDARD
                .decode(self.contract_abi.clone())?
                .as_slice(),
        )?;
        let event = abi.event(&self.event_name)?;
        let log = event.parse_log(RawLog {
            topics,
            data: data.0,
        })?;

        let mut event_attrs = HashMap::new();
        for attr in log.params {
            event_attrs.insert(attr.name, attr.value.to_string());
        }

        for (attr_name, attr_value) in &self.event_attributes {
            if *event_attrs.get(attr_name).ok_or("could not find attr")? != *attr_value {
                return Err("exepected attr not found".into());
            }
        }

        Ok(AlertNotificationData::ScEvents(
            SmartContractEventsNotificationData {
                contract_addr: got_contract_addr,
                event_name: self.event_name.clone(),
                event_attributes: self.event_attributes.clone(),
                tx_hash: ctx.tx_hash,
            },
        ))
    }
}

impl AlertSourceFilter for AlertEthMonitorFunds {
    fn filter(&self, alert_source: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = alert_source.get_eth_tx()?;
        let from = ctx.tx_resp.from.hash_string()?;

        if ctx.tx_resp.status.unwrap_or_default().as_u64() == 0 {
            return Err("transaction did not succeed".into());
        }

        let address = self.address.to_ascii_lowercase();

        if ctx.tx.input.0.is_empty() && !ctx.tx.value.is_zero() {
            let to = ctx
                .tx_resp
                .to
                .ok_or("could not find to addr in eth tx")?
                .hash_string()?;

            let notification =
                AlertNotificationData::MonitorFundsCoin(MonitorFundsCoinNotificationData {
                    from: from.clone(),
                    to: to.clone(),
                    amount: vec![CoinAmount {
                        amount: ctx.tx.value.to_string(),
                        denom: String::new(),
                    }],
                    tx_hash: ctx.tx_hash,
                });

            if address.is_empty() {
                return Ok(notification);
            }

            if address == from || address == to {
                return Ok(notification);
            }

            return Err("address not related to coin call".into());
        }

        let input_data = ctx.tx.input.clone().0;

        let erc_20 = web3::ethabi::Contract::load(ERC20_ABI.as_bytes())?;
        let function = erc_20.function("transfer")?;
        let sig = function.short_signature();
        let args = function.decode_input(
            input_data
                .strip_prefix(&sig)
                .ok_or("invalid function sig")?,
        )?;
        let to = args
            .get(0)
            .ok_or("could not get to address")?
            .clone()
            .into_address()
            .ok_or("could not convert token into address")?
            .hash_string()?;
        let amount = args
            .get(1)
            .ok_or("could not get to address")?
            .clone()
            .into_uint()
            .ok_or("could not convert token into address")?;

        let notification =
            AlertNotificationData::MonitorFundsToken(MonitorFundsTokenNotificationData {
                from: from.clone(),
                to: to.clone(),
                amount: amount.to_string(),
                tx_hash: ctx.tx_hash,
                contract_addr: ctx
                    .tx
                    .to
                    .ok_or("could not find eth contract addr")?
                    .hash_string()?,
            });

        if address.is_empty() {
            return Ok(notification);
        }

        if address == from || address == to {
            return Ok(notification);
        }

        Err("address not related to token call".into())
    }
}

impl AlertSourceFilter for AlertEthTxOutcome {
    fn filter(&self, alert_source: &ProcessAlertSourceRequeust) -> Result<AlertNotificationData> {
        let ctx = alert_source.get_eth_tx()?;
        let from = ctx.tx_resp.from.hash_string()?;
        let signer = self.signer.to_ascii_lowercase();
        if !signer.is_empty() && from != signer {
            return Err("signer mismatch".into());
        }

        let status = ctx
            .tx_resp
            .status
            .ok_or("could not find status code in eth tx")?
            .as_u64();
        let matched =
            match EthTxOutcome::from_i32(self.outcome).ok_or("invalid i32 val for enum")? {
                EthTxOutcome::Succeeded => status == 1,
                EthTxOutcome::Failed => status == 0,
            };

        if !matched {
            return Err("tx outcome mismatch".into());
        }

        Ok(AlertNotificationData::TxOutcome(
            TxOutcomeNotificationData {
                signer: self.signer.clone(),
                outcome: self.outcome().as_str_name().to_string(),
                tx_hash: ctx.tx_hash,
            },
        ))
    }
}
