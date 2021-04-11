use thiserror::Error;

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
    #[error("Unknown error happened.")]
    UnknownError
}
