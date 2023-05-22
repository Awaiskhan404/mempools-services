use mempools_api::api::{
    alert_notification_data::AlertNotificationData, MonitorFundsCoinNotificationData,
    MonitorFundsTokenNotificationData, SmartContractEventsNotificationData,
    TxOutcomeNotificationData, UserAlert,
};
use util::convert::TryConvert;

use util::Result;

pub trait NotificationContent: Send + Sync {
    fn email_sub(&self) -> String;

    fn telegram(&self) -> String;
    fn email(&self) -> String;
    fn webhook(&self) -> String;
}

pub trait AlertNotificationContent: Send + Sync {
    fn email_sub(&self, alert: UserAlert) -> String;

    fn telegram(&self, alert: UserAlert) -> String;
    fn email(&self, alert: UserAlert) -> String;
    fn webhook(&self, alert: UserAlert) -> String;
}

impl TryConvert<Box<dyn AlertNotificationContent>> for AlertNotificationData {
    fn try_convert(&self) -> Result<Box<dyn AlertNotificationContent>> {
        Ok(match self.clone() {
            AlertNotificationData::MonitorFundsCoin(n) => Box::new(n),
            AlertNotificationData::MonitorFundsToken(n) => Box::new(n),
            AlertNotificationData::TxOutcome(n) => Box::new(n),
            AlertNotificationData::ScEvents(n) => Box::new(n),
        })
    }
}

pub struct AlertNotificationContentImplementation(
    pub Box<dyn AlertNotificationContent>,
    pub UserAlert,
);

impl NotificationContent for AlertNotificationContentImplementation {
    fn email_sub(&self) -> String {
        self.0.email_sub(self.1.clone())
    }

    fn telegram(&self) -> String {
        self.0.telegram(self.1.clone())
    }

    fn email(&self) -> String {
        self.0.email(self.1.clone())
    }

    fn webhook(&self) -> String {
        self.0.webhook(self.1.clone())
    }
}

pub struct RateLimitNotification;

impl NotificationContent for RateLimitNotification {
    fn email_sub(&self) -> String {
        "Mempools daily notification limit reached".to_string()
    }

    fn telegram(&self) -> String {
        "Your daily notification limit has been reached on mempools".to_string()
    }

    fn email(&self) -> String {
        "Your daily notification limit has been reached on mempools".to_string()
    }

    fn webhook(&self) -> String {
        "Your daily notification limit has been reached on mempools".to_string()
    }
}

pub struct ConfirmationCodeNotificationContent(pub String, pub String);

impl NotificationContent for ConfirmationCodeNotificationContent {
    fn email_sub(&self) -> String {
        "Mempools - Destination Registration Confirmation Code".to_string()
    }

    fn telegram(&self) -> String {
        format!("Here is the confirmation code related to the registrations of destination {} - Confirmation Code - {}", self.0, self.1)
    }

    fn email(&self) -> String {
        format!("Here is the confirmation code related to the registrations of destination {} - Confirmation Code - {}", self.0, self.1)
    }

    fn webhook(&self) -> String {
        format!("Here is the confirmation code related to the registrations of destination {} - Confirmation Code - {}", self.0, self.1)
    }
}

impl AlertNotificationContent for MonitorFundsCoinNotificationData {
    fn email_sub(&self, alert: UserAlert) -> String {
        format!("Mempools alert - {} - {}", alert.id, self.tx_hash)
    }

    fn telegram(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 coin {:?}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.amount, self.tx_hash
        )
    }

    fn email(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 coin {:?}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.amount, self.tx_hash
        )
    }

    fn webhook(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 coin {:?}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.amount, self.tx_hash
        )
    }
}

impl AlertNotificationContent for MonitorFundsTokenNotificationData {
    fn email_sub(&self, alert: UserAlert) -> String {
        format!("Mempools alert - {} - {}", alert.id, self.tx_hash)
    }

    fn telegram(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 token {} of amount {}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.contract_addr, self.amount, self.tx_hash
        )
    }

    fn email(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 token {} of amount {}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.contract_addr, self.amount, self.tx_hash
        )
    }

    fn webhook(&self, _alert: UserAlert) -> String {
        format!(
            "🔥 Mempools update🔥 

        📈 A transaction has been made from {} to {} for 💰 token {} of amount {}. 
        
        The transaction ID is 🔍 
        {}",
            self.from, self.to, self.contract_addr, self.amount, self.tx_hash
        )
    }
}

impl AlertNotificationContent for TxOutcomeNotificationData {
    fn email_sub(&self, alert: UserAlert) -> String {
        format!("Mempools alert - {} - {}", alert.id, self.tx_hash)
    }

    fn telegram(&self, _alert: UserAlert) -> String {
        format!(
            "A transaction from signer {} was executed which {}. Here is the transaction hash {}",
            self.signer, self.outcome, self.tx_hash
        )
    }

    fn email(&self, _alert: UserAlert) -> String {
        format!(
            "A transaction from signer {} was executed which {}. Here is the transaction hash {}",
            self.signer, self.outcome, self.tx_hash
        )
    }

    fn webhook(&self, _alert: UserAlert) -> String {
        format!(
            "A transaction from signer {} was executed which {}. Here is the transaction hash {}",
            self.signer, self.outcome, self.tx_hash
        )
    }
}

impl AlertNotificationContent for SmartContractEventsNotificationData {
    fn email_sub(&self, alert: UserAlert) -> String {
        format!("Mempools alert - {} - {}", alert.id, self.tx_hash)
    }

    fn telegram(&self, _alert: UserAlert) -> String {
        format!(
            "Event name {} with attributes {:?} occured in smart contract {} in tx hash {}",
            self.event_name, self.event_attributes, self.contract_addr, self.tx_hash
        )
    }

    fn email(&self, _alert: UserAlert) -> String {
        format!(
            "Event name {} with attributes {:?} occured in smart contract {} in tx hash {}",
            self.event_name, self.event_attributes, self.contract_addr, self.tx_hash
        )
    }

    fn webhook(&self, _alert: UserAlert) -> String {
        format!(
            "Event name {} with attributes {:?} occured in smart contract {} in tx hash {}",
            self.event_name, self.event_attributes, self.contract_addr, self.tx_hash
        )
    }
}
