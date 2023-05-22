#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAlertRequest {
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<UserAlert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAlertResponse {
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<UserAlert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlertRequest {
    #[prost(string, tag = "1")]
    pub alert_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlertResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserDestinationRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserDestinationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDestinationRequest {
    #[prost(message, optional, tag = "1")]
    pub destination: ::core::option::Option<UserDestination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDestinationResponse {
    #[prost(message, optional, tag = "1")]
    pub destination: ::core::option::Option<UserDestination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationsRequest {
    #[prost(uint64, tag = "1")]
    pub page: u64,
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<AlertNotification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitCognitoCodeGrantRequest {
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitCognitoCodeGrantResponse {
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChainRequest {
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub chain_data: ::core::option::Option<ChainData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChainResponse {
    #[prost(message, optional, tag = "1")]
    pub chain: ::core::option::Option<Chain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChainRequest {
    #[prost(message, optional, tag = "1")]
    pub chain: ::core::option::Option<Chain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChainResponse {
    #[prost(message, optional, tag = "1")]
    pub chain: ::core::option::Option<Chain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainsResponse {
    #[prost(message, repeated, tag = "1")]
    pub chains: ::prost::alloc::vec::Vec<Chain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chain {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    #[prost(enumeration = "chain::Status", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub chain_data: ::core::option::Option<ChainData>,
}
/// Nested message and enum types in `Chain`.
pub mod chain {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Enabled = 0,
        Disabled = 1,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Enabled => "ENABLED",
                Status::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainData {
    #[prost(oneof = "chain_data::ChainData", tags = "1, 2, 3")]
    pub chain_data: ::core::option::Option<chain_data::ChainData>,
}
/// Nested message and enum types in `ChainData`.
pub mod chain_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ChainData {
        #[prost(message, tag = "1")]
        CosmosChainData(super::CosmosChainData),
        #[prost(message, tag = "2")]
        CosmosEvmChainData(super::CosmosEvmChainData),
        #[prost(message, tag = "3")]
        EthChainData(super::EthChainData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosChainData {
    #[prost(string, tag = "1")]
    pub grpc_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bech32_prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosEvmChainData {
    #[prost(message, optional, tag = "1")]
    pub cosmos_chain_data: ::core::option::Option<CosmosChainData>,
    #[prost(message, optional, tag = "2")]
    pub eth_chain_data: ::core::option::Option<EthChainData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthChainData {
    #[prost(string, tag = "1")]
    pub eth_rpc_endpoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserDestinationsRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub page: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserDestinationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_destinations: ::prost::alloc::vec::Vec<UserDestination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateDestinationRegistrationRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<Destination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateDestinationRegistrationResponse {
    #[prost(string, tag = "1")]
    pub destination_registration_proposal_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmDestinationRegistrationRequest {
    #[prost(string, tag = "1")]
    pub destination_registration_proposal_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub confirmation_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmDestinationRegistrationResponse {
    #[prost(message, optional, tag = "1")]
    pub destination: ::core::option::Option<UserDestination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDestination {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub destination: ::core::option::Option<Destination>,
    #[prost(enumeration = "user_destination::Status", tag = "4")]
    pub status: i32,
}
/// Nested message and enum types in `UserDestination`.
pub mod user_destination {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Enabled = 0,
        Disabled = 1,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Enabled => "ENABLED",
                Status::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationRegistrationProposal {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub confirmation_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub destination: ::core::option::Option<Destination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertsRequest {
    #[prost(uint64, tag = "1")]
    pub page: u64,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertsResponse {
    #[prost(message, repeated, tag = "1")]
    pub alerts: ::prost::alloc::vec::Vec<UserAlert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlertRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub alert: ::core::option::Option<Alert>,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub destination_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlertResponse {
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<UserAlert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAlert {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub alert: ::core::option::Option<Alert>,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub destination_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "user_alert::Status", tag = "6")]
    pub status: i32,
    #[prost(string, tag = "7")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(enumeration = "AlertSource", tag = "8")]
    pub alert_source: i32,
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `UserAlert`.
pub mod user_alert {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Enabled = 0,
        Disabled = 1,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Enabled => "Enabled",
                Status::Disabled => "Disabled",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Enabled" => Some(Self::Enabled),
                "Disabled" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    #[prost(oneof = "alert::ChainAlert", tags = "1, 2, 3")]
    pub chain_alert: ::core::option::Option<alert::ChainAlert>,
}
/// Nested message and enum types in `Alert`.
pub mod alert {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ChainAlert {
        #[prost(message, tag = "1")]
        CosmosAlert(super::CosmosAlert),
        #[prost(message, tag = "2")]
        CosmosEvmAlert(super::CosmosEvmAlert),
        #[prost(message, tag = "3")]
        EthAlert(super::EthAlert),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosAlert {
    #[prost(oneof = "cosmos_alert::CosmosAlert", tags = "1, 2, 3, 4")]
    pub cosmos_alert: ::core::option::Option<cosmos_alert::CosmosAlert>,
}
/// Nested message and enum types in `CosmosAlert`.
pub mod cosmos_alert {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CosmosAlert {
        #[prost(message, tag = "1")]
        AlertCosmosSendFunds(super::AlertCosmosSendFunds),
        #[prost(message, tag = "2")]
        AlertCosmosMonitorFunds(super::AlertCosmosMonitorFunds),
        #[prost(message, tag = "3")]
        AlertCosmosSmartContractEvents(super::AlertCosmosSmartContractEvents),
        #[prost(message, tag = "4")]
        AlertCosmosTxOutcome(super::AlertCosmosTxOutcome),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosEvmAlert {
    #[prost(oneof = "cosmos_evm_alert::CosmosEvmAlert", tags = "1, 2, 3, 4, 5")]
    pub cosmos_evm_alert: ::core::option::Option<cosmos_evm_alert::CosmosEvmAlert>,
}
/// Nested message and enum types in `CosmosEvmAlert`.
pub mod cosmos_evm_alert {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CosmosEvmAlert {
        #[prost(message, tag = "1")]
        AlertEthMonitorFunds(super::AlertEthMonitorFunds),
        #[prost(message, tag = "2")]
        AlertEthTxOutcome(super::AlertEthTxOutcome),
        #[prost(message, tag = "3")]
        AlertEthSmartContractEvents(super::AlertEthSmartContractEvents),
        #[prost(message, tag = "4")]
        AlertCosmosMonitorFunds(super::AlertCosmosMonitorFunds),
        #[prost(message, tag = "5")]
        AlertCosmosTxOutcome(super::AlertCosmosTxOutcome),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAlert {
    #[prost(oneof = "eth_alert::EthAlert", tags = "1, 2, 3")]
    pub eth_alert: ::core::option::Option<eth_alert::EthAlert>,
}
/// Nested message and enum types in `EthAlert`.
pub mod eth_alert {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EthAlert {
        #[prost(message, tag = "1")]
        AlertEthMonitorFunds(super::AlertEthMonitorFunds),
        #[prost(message, tag = "2")]
        AlertEthTxOutcome(super::AlertEthTxOutcome),
        #[prost(message, tag = "3")]
        AlertEthSmartContractEvents(super::AlertEthSmartContractEvents),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertEthSmartContractEvents {
    #[prost(string, tag = "1")]
    pub contract_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_abi: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "4")]
    pub event_attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertEthMonitorFunds {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertEthTxOutcome {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(enumeration = "alert_eth_tx_outcome::EthTxOutcome", tag = "2")]
    pub outcome: i32,
}
/// Nested message and enum types in `AlertEthTxOutcome`.
pub mod alert_eth_tx_outcome {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EthTxOutcome {
        Succeeded = 0,
        Failed = 1,
    }
    impl EthTxOutcome {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EthTxOutcome::Succeeded => "SUCCEEDED",
                EthTxOutcome::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertCosmosSmartContractEvents {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub event_attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "3")]
    pub event_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertCosmosMonitorFunds {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertCosmosSendFunds {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertCosmosTxOutcome {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(enumeration = "alert_cosmos_tx_outcome::CosmosTxOutcome", tag = "2")]
    pub outcome: i32,
}
/// Nested message and enum types in `AlertCosmosTxOutcome`.
pub mod alert_cosmos_tx_outcome {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CosmosTxOutcome {
        Succeeded = 0,
        Failed = 1,
    }
    impl CosmosTxOutcome {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CosmosTxOutcome::Succeeded => "SUCCEEDED",
                CosmosTxOutcome::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    #[prost(oneof = "destination::Destination", tags = "1, 2, 3, 4")]
    pub destination: ::core::option::Option<destination::Destination>,
}
/// Nested message and enum types in `Destination`.
pub mod destination {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        #[prost(string, tag = "1")]
        Email(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Webhook(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Telegram(::prost::alloc::string::String),
        #[prost(string, tag = "4")]
        DiscordWebhook(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertNotification {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub notification_data: ::core::option::Option<AlertNotificationData>,
    #[prost(string, tag = "3")]
    pub alert_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub alert_source_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertNotificationData {
    #[prost(
        oneof = "alert_notification_data::AlertNotificationData",
        tags = "1, 2, 3, 4"
    )]
    pub alert_notification_data: ::core::option::Option<
        alert_notification_data::AlertNotificationData,
    >,
}
/// Nested message and enum types in `AlertNotificationData`.
pub mod alert_notification_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AlertNotificationData {
        #[prost(message, tag = "1")]
        MonitorFundsCoin(super::MonitorFundsCoinNotificationData),
        #[prost(message, tag = "2")]
        MonitorFundsToken(super::MonitorFundsTokenNotificationData),
        #[prost(message, tag = "3")]
        TxOutcome(super::TxOutcomeNotificationData),
        #[prost(message, tag = "4")]
        ScEvents(super::SmartContractEventsNotificationData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContractEventsNotificationData {
    #[prost(string, tag = "1")]
    pub contract_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub event_attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "4")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutcomeNotificationData {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub outcome: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitorFundsCoinNotificationData {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<
        monitor_funds_coin_notification_data::CoinAmount,
    >,
    #[prost(string, tag = "4")]
    pub tx_hash: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MonitorFundsCoinNotificationData`.
pub mod monitor_funds_coin_notification_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CoinAmount {
        #[prost(string, tag = "1")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub denom: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitorFundsTokenNotificationData {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrawlerData {
    #[prost(oneof = "crawler_data::CrawlerData", tags = "1, 2, 3")]
    pub crawler_data: ::core::option::Option<crawler_data::CrawlerData>,
}
/// Nested message and enum types in `CrawlerData`.
pub mod crawler_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CrawlerData {
        #[prost(message, tag = "1")]
        Cosmos(super::CosmosCrawlerData),
        #[prost(message, tag = "2")]
        CosmosEvm(super::CosmosEvmCrawlerData),
        #[prost(message, tag = "3")]
        Ethereum(super::EthCrawlerData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosCrawlerData {
    #[prost(uint64, tag = "1")]
    pub processed_blocks: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosEvmCrawlerData {
    #[prost(message, optional, tag = "1")]
    pub cosmos: ::core::option::Option<CosmosCrawlerData>,
    #[prost(message, optional, tag = "2")]
    pub ethereum: ::core::option::Option<EthCrawlerData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthCrawlerData {
    #[prost(uint64, tag = "1")]
    pub processed_blocks: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlertSource {
    CosmosMsg = 0,
    CosmosTx = 1,
    EthLog = 2,
    EthTx = 3,
}
impl AlertSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AlertSource::CosmosMsg => "CosmosMsg",
            AlertSource::CosmosTx => "CosmosTx",
            AlertSource::EthLog => "EthLog",
            AlertSource::EthTx => "EthTx",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CosmosMsg" => Some(Self::CosmosMsg),
            "CosmosTx" => Some(Self::CosmosTx),
            "EthLog" => Some(Self::EthLog),
            "EthTx" => Some(Self::EthTx),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod gateway_public_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GatewayPublicClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayPublicClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayPublicClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayPublicClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GatewayPublicClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn submit_cognito_code_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitCognitoCodeGrantRequest>,
        ) -> Result<
            tonic::Response<super::SubmitCognitoCodeGrantResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.GatewayPublic/SubmitCognitoCodeGrant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod gateway_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GatewayAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayAdminClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayAdminClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayAdminClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GatewayAdminClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn create_chain(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChainRequest>,
        ) -> Result<tonic::Response<super::CreateChainResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.GatewayAdmin/CreateChain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_chain(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChainRequest>,
        ) -> Result<tonic::Response<super::UpdateChainResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.GatewayAdmin/UpdateChain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod gateway_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GatewayClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GatewayClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Destinations
        pub async fn initiate_destination_registration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::InitiateDestinationRegistrationRequest,
            >,
        ) -> Result<
            tonic::Response<super::InitiateDestinationRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/InitiateDestinationRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn confirm_destination_registration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ConfirmDestinationRegistrationRequest,
            >,
        ) -> Result<
            tonic::Response<super::ConfirmDestinationRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/ConfirmDestinationRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_destinations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserDestinationsRequest>,
        ) -> Result<tonic::Response<super::GetUserDestinationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/GetUserDestinations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_user_destination(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserDestinationRequest>,
        ) -> Result<
            tonic::Response<super::UpdateUserDestinationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/UpdateUserDestination",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_user_destination(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserDestinationRequest>,
        ) -> Result<
            tonic::Response<super::DeleteUserDestinationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/DeleteUserDestination",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Alerts
        pub async fn create_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAlertRequest>,
        ) -> Result<tonic::Response<super::CreateAlertResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Gateway/CreateAlert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_alerts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAlertsRequest>,
        ) -> Result<tonic::Response<super::GetAlertsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Gateway/GetAlerts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAlertRequest>,
        ) -> Result<tonic::Response<super::UpdateAlertResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Gateway/UpdateAlert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAlertRequest>,
        ) -> Result<tonic::Response<super::DeleteAlertResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Gateway/DeleteAlert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Notifications
        pub async fn get_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationsRequest>,
        ) -> Result<tonic::Response<super::GetNotificationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Gateway/GetNotifications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Chains
        pub async fn get_chains(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChainsRequest>,
        ) -> Result<tonic::Response<super::GetChainsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Gateway/GetChains");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod gateway_public_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GatewayPublicServer.
    #[async_trait]
    pub trait GatewayPublic: Send + Sync + 'static {
        async fn submit_cognito_code_grant(
            &self,
            request: tonic::Request<super::SubmitCognitoCodeGrantRequest>,
        ) -> Result<
            tonic::Response<super::SubmitCognitoCodeGrantResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayPublicServer<T: GatewayPublic> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GatewayPublic> GatewayPublicServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GatewayPublicServer<T>
    where
        T: GatewayPublic,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.GatewayPublic/SubmitCognitoCodeGrant" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitCognitoCodeGrantSvc<T: GatewayPublic>(pub Arc<T>);
                    impl<
                        T: GatewayPublic,
                    > tonic::server::UnaryService<super::SubmitCognitoCodeGrantRequest>
                    for SubmitCognitoCodeGrantSvc<T> {
                        type Response = super::SubmitCognitoCodeGrantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitCognitoCodeGrantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).submit_cognito_code_grant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitCognitoCodeGrantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GatewayPublic> Clone for GatewayPublicServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GatewayPublic> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayPublic> tonic::server::NamedService for GatewayPublicServer<T> {
        const NAME: &'static str = "api.GatewayPublic";
    }
}
/// Generated server implementations.
pub mod gateway_admin_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GatewayAdminServer.
    #[async_trait]
    pub trait GatewayAdmin: Send + Sync + 'static {
        async fn create_chain(
            &self,
            request: tonic::Request<super::CreateChainRequest>,
        ) -> Result<tonic::Response<super::CreateChainResponse>, tonic::Status>;
        async fn update_chain(
            &self,
            request: tonic::Request<super::UpdateChainRequest>,
        ) -> Result<tonic::Response<super::UpdateChainResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GatewayAdminServer<T: GatewayAdmin> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GatewayAdmin> GatewayAdminServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GatewayAdminServer<T>
    where
        T: GatewayAdmin,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.GatewayAdmin/CreateChain" => {
                    #[allow(non_camel_case_types)]
                    struct CreateChainSvc<T: GatewayAdmin>(pub Arc<T>);
                    impl<
                        T: GatewayAdmin,
                    > tonic::server::UnaryService<super::CreateChainRequest>
                    for CreateChainSvc<T> {
                        type Response = super::CreateChainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateChainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_chain(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateChainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.GatewayAdmin/UpdateChain" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateChainSvc<T: GatewayAdmin>(pub Arc<T>);
                    impl<
                        T: GatewayAdmin,
                    > tonic::server::UnaryService<super::UpdateChainRequest>
                    for UpdateChainSvc<T> {
                        type Response = super::UpdateChainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateChainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_chain(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateChainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GatewayAdmin> Clone for GatewayAdminServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GatewayAdmin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayAdmin> tonic::server::NamedService for GatewayAdminServer<T> {
        const NAME: &'static str = "api.GatewayAdmin";
    }
}
/// Generated server implementations.
pub mod gateway_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GatewayServer.
    #[async_trait]
    pub trait Gateway: Send + Sync + 'static {
        /// Destinations
        async fn initiate_destination_registration(
            &self,
            request: tonic::Request<super::InitiateDestinationRegistrationRequest>,
        ) -> Result<
            tonic::Response<super::InitiateDestinationRegistrationResponse>,
            tonic::Status,
        >;
        async fn confirm_destination_registration(
            &self,
            request: tonic::Request<super::ConfirmDestinationRegistrationRequest>,
        ) -> Result<
            tonic::Response<super::ConfirmDestinationRegistrationResponse>,
            tonic::Status,
        >;
        async fn get_user_destinations(
            &self,
            request: tonic::Request<super::GetUserDestinationsRequest>,
        ) -> Result<tonic::Response<super::GetUserDestinationsResponse>, tonic::Status>;
        async fn update_user_destination(
            &self,
            request: tonic::Request<super::UpdateUserDestinationRequest>,
        ) -> Result<
            tonic::Response<super::UpdateUserDestinationResponse>,
            tonic::Status,
        >;
        async fn delete_user_destination(
            &self,
            request: tonic::Request<super::DeleteUserDestinationRequest>,
        ) -> Result<
            tonic::Response<super::DeleteUserDestinationResponse>,
            tonic::Status,
        >;
        /// Alerts
        async fn create_alert(
            &self,
            request: tonic::Request<super::CreateAlertRequest>,
        ) -> Result<tonic::Response<super::CreateAlertResponse>, tonic::Status>;
        async fn get_alerts(
            &self,
            request: tonic::Request<super::GetAlertsRequest>,
        ) -> Result<tonic::Response<super::GetAlertsResponse>, tonic::Status>;
        async fn update_alert(
            &self,
            request: tonic::Request<super::UpdateAlertRequest>,
        ) -> Result<tonic::Response<super::UpdateAlertResponse>, tonic::Status>;
        async fn delete_alert(
            &self,
            request: tonic::Request<super::DeleteAlertRequest>,
        ) -> Result<tonic::Response<super::DeleteAlertResponse>, tonic::Status>;
        /// Notifications
        async fn get_notifications(
            &self,
            request: tonic::Request<super::GetNotificationsRequest>,
        ) -> Result<tonic::Response<super::GetNotificationsResponse>, tonic::Status>;
        /// Chains
        async fn get_chains(
            &self,
            request: tonic::Request<super::GetChainsRequest>,
        ) -> Result<tonic::Response<super::GetChainsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GatewayServer<T: Gateway> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Gateway> GatewayServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GatewayServer<T>
    where
        T: Gateway,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.Gateway/InitiateDestinationRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct InitiateDestinationRegistrationSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<
                        super::InitiateDestinationRegistrationRequest,
                    > for InitiateDestinationRegistrationSvc<T> {
                        type Response = super::InitiateDestinationRegistrationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::InitiateDestinationRegistrationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).initiate_destination_registration(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitiateDestinationRegistrationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/ConfirmDestinationRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmDestinationRegistrationSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<
                        super::ConfirmDestinationRegistrationRequest,
                    > for ConfirmDestinationRegistrationSvc<T> {
                        type Response = super::ConfirmDestinationRegistrationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ConfirmDestinationRegistrationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).confirm_destination_registration(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmDestinationRegistrationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/GetUserDestinations" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserDestinationsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::GetUserDestinationsRequest>
                    for GetUserDestinationsSvc<T> {
                        type Response = super::GetUserDestinationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserDestinationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_destinations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserDestinationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/UpdateUserDestination" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserDestinationSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::UpdateUserDestinationRequest>
                    for UpdateUserDestinationSvc<T> {
                        type Response = super::UpdateUserDestinationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserDestinationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_user_destination(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateUserDestinationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/DeleteUserDestination" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserDestinationSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::DeleteUserDestinationRequest>
                    for DeleteUserDestinationSvc<T> {
                        type Response = super::DeleteUserDestinationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserDestinationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_user_destination(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteUserDestinationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/CreateAlert" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAlertSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::CreateAlertRequest>
                    for CreateAlertSvc<T> {
                        type Response = super::CreateAlertResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAlertRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_alert(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAlertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/GetAlerts" => {
                    #[allow(non_camel_case_types)]
                    struct GetAlertsSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::GetAlertsRequest>
                    for GetAlertsSvc<T> {
                        type Response = super::GetAlertsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAlertsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_alerts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAlertsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/UpdateAlert" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAlertSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::UpdateAlertRequest>
                    for UpdateAlertSvc<T> {
                        type Response = super::UpdateAlertResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAlertRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_alert(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAlertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/DeleteAlert" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAlertSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::DeleteAlertRequest>
                    for DeleteAlertSvc<T> {
                        type Response = super::DeleteAlertResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAlertRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_alert(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAlertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/GetNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct GetNotificationsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::GetNotificationsRequest>
                    for GetNotificationsSvc<T> {
                        type Response = super::GetNotificationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNotificationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_notifications(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNotificationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Gateway/GetChains" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainsSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::GetChainsRequest>
                    for GetChainsSvc<T> {
                        type Response = super::GetChainsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetChainsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_chains(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetChainsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Gateway> Clone for GatewayServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Gateway> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Gateway> tonic::server::NamedService for GatewayServer<T> {
        const NAME: &'static str = "api.Gateway";
    }
}
