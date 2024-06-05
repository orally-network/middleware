use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug, CandidType, Deserialize)]
pub enum MiddlewareError {
    #[error("Failed to send cycles: {0}")]
    FailedToSendCycles(String),
    #[error("Chain not found: {0}")]
    MiddlewareNotFound(u64),
    #[error("Chain already exists: {0}")]
    CommunicationWithMiddlewareInstanceFailed(String),
    #[error("Failed to get canister status: {0}")]
    FailedToGetCanisterStatus(String),
    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),

    #[error("Failed to update settings: {0}")]
    FailedToUpdateSettings(String),
    #[error("Failed to create: {0}")]
    FailedToCreate(String),
    #[error("Failed to stop: {0}")]
    FailedToStop(String),
    #[error("Failed to delete: {0}")]
    FailedToDelete(String),
    #[error("Failed to install code: {0}")]
    FailedToInstallCode(String),
    #[error("Failed to upgrade: {0}")]
    FailedToUpgrade(String),
    #[error("Middleware instance error: {0}")]
    MiddlewareInstanceError(String),
}

#[derive(Error, Debug, CandidType, PartialEq, Deserialize)]
pub enum UtilsError {
    #[error("Timer is not initialized")]
    TimerIsNotInitialized,
    #[error("Unable to get asset data: {0}")]
    UnableToGetAssetData(String),
    #[error("Invalid address format: {0}")]
    InvalidAddressFormat(String),
    #[error("Invalid SIWE message: {0}")]
    InvalidSIWEMessage(String),
    #[error("From hex error: {0}")]
    FromHexError(String),
    #[error("Failed to get apollo evm address: {0}")]
    FailedToGetmiddlewareEvmAddress(String),
    #[error("Not a controller")]
    NotAController,
    #[error("Unable to get random: {0}")]
    UnableToGetRandom(String),
}

#[derive(Error, Debug, CandidType, PartialEq, Deserialize)]
#[error("Canister communication error: {message}")]
pub struct CanisterCommunicationError {
    pub message: String,
}

impl From<(RejectionCode, String)> for CanisterCommunicationError {
    fn from((_, message): (RejectionCode, String)) -> Self {
        Self { message }
    }
}
