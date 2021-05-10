use thiserror::Error;
use crate::db::TelegramAccount;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("This node is not avaiable now.")]
    NodeNotAvailable,
    #[error("No signer to sign this transaction.")]
    NoSigner,
    #[error("This is an invalid ss58 address.")]
    InvalidAccountId,
    #[error("Please provide valid wasm binary.")]
    SetCodeFailure(#[from] subxt::Error),
    #[error("Decode event error `{0}`")]
    DecodeEventError(#[from] codec::Error),
    #[error("No event found.")]
    NoEventFound,
    #[error("Sled Db Error `{0}.")]
    SledDbError(#[from] sled::Error),
    #[error("Unknown error happened.")]
    UnknownError,
    #[error("`{0}` has reached their daily quota. Only request once per day.")]
    ReachQuota(TelegramAccount),
    #[error("There's error on bot configuration.")]
    ErrorBotConfig,
}
