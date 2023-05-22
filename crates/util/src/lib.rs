use std::{error::Error, u8};

use base64::Engine;

use cosmrs::bip32::secp256k1::sha2::Sha256;
use cosmrs::{bip32::secp256k1::sha2::Digest, tx::SignerInfo};

use serde::{Deserialize, Serialize};

use tonic::Status;
use web3::types::{H160, H256};
pub mod clients;
pub mod convert;
pub mod service_registry;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub trait ToGrpcResult<T> {
    fn to_grpc_result(self) -> core::result::Result<T, Status>;
}

impl<T, E: Into<Box<dyn Error + Send + Sync>>> ToGrpcResult<T> for core::result::Result<T, E> {
    fn to_grpc_result(self) -> core::result::Result<T, Status> {
        self.map_err(|err| {
            let err: Box<dyn Error> = err.into();
            Status::internal(format!("Unexpected error: {}", err))
        })
    }
}

pub trait ToResult<T> {
    fn to_result(self) -> Result<T>;
}

impl<T, E: ToString> ToResult<T> for core::result::Result<T, E> {
    fn to_result(self) -> Result<T> {
        self.map_err(|err| err.to_string().into())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CognitoClaims {
    pub at_hash: String,
    pub sub: String,
    pub email_verified: bool,
    pub iss: String,
    #[serde(rename = "cognito:username")]
    pub username: String,
    pub origin_jti: String,
    pub aud: String,
    pub token_use: String,
    pub auth_time: i64,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub email: String,
}

pub trait GetClaims {
    fn get_claims(&self) -> Result<CognitoClaims>;
}

impl<T> GetClaims for tonic::Request<T> {
    fn get_claims(&self) -> Result<CognitoClaims> {
        let (_, bearer_token) = self
            .metadata()
            .get("Authorization")
            .ok_or("could not find Authorization header to get claims")?
            .to_str()?
            .split_once(' ')
            .ok_or("incorrect authorization header format - must be in bearer format")?;
        let parts = bearer_token.split('.').collect::<Vec<&str>>();
        let base64_encoded_claims = parts.get(1).ok_or("invalid id token format")?;
        let json_claims = base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(base64_encoded_claims)?;
        let claims = serde_json::from_slice(&json_claims)?;
        Ok(claims)
    }
}

pub fn get_sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hex::encode(hasher.finalize())
}

// pub fn cosmos_to_eth_address(cosmos_addr: &str) -> Result<String> {
//     let (_, data) = subtle_encoding::bech32::decode(cosmos_addr)?;
//     Ok(clarity::utils::bytes_to_hex_str(data.as_slice()))
// }

// pub fn eth_to_cosmos_address(bech32_prefix: &str, eth_addr: &str) -> Result<String> {
//     let data = clarity::utils::hex_str_to_bytes(eth_addr)?;
//     let cosmos_addr = subtle_encoding::bech32::encode(bech32_prefix, data);
//     Ok(cosmos_addr)
// }

pub trait HashString {
    fn hash_string(&self) -> Result<String>;
}

impl HashString for H160 {
    fn hash_string(&self) -> Result<String> {
        Ok(format!("{:#x}", self))
    }
}

impl HashString for H256 {
    fn hash_string(&self) -> Result<String> {
        Ok(format!("{:#x}", self))
    }
}

pub fn get_signers_from_tx(chain_prefix: String, tx: cosmrs::Tx) -> Vec<String> {
    let mut all_signers = vec![];
    for info in tx.auth_info.signer_infos {
        if let Ok(signers) = signer_info_to_account_id(info, chain_prefix.clone()) {
            signers
                .iter()
                .for_each(|signer| all_signers.push(signer.clone()));
        }
    }

    all_signers
}

fn signer_info_to_account_id(info: SignerInfo, chain_prefix: String) -> Result<Vec<String>> {
    let temp = info.public_key.ok_or("could not get pub key for signer")?;
    let signers = match temp {
        cosmrs::tx::SignerPublicKey::Single(s) => {
            vec![s.account_id(&chain_prefix)?.to_string()]
        }
        cosmrs::tx::SignerPublicKey::LegacyAminoMultisig(s) => {
            let pks = s.public_keys;

            let mut signers = vec![];
            for pk in pks {
                signers.push(pk.account_id(&chain_prefix)?.to_string())
            }

            signers
        }
        cosmrs::tx::SignerPublicKey::Any(_a) => return Err("unexpected signer type".into()),
    };

    Ok(signers)
}
