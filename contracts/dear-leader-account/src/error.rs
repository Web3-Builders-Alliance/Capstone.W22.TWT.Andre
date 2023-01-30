use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Wrong token sent")]
    WrongToken {},

    #[error("The address sent is invalid")]
    InvalidAddr {},

    #[error("No delegation found")]
    NoDelegation {},

    #[error("Requested undelegation amount is too high")]
    UndelegateAmountTooHigh {},

    #[error("Requested redelegation amount is too high")]
    RedelegateAmountTooHigh {},

    #[error("Vote option is invalid")]
    InvalidVote {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
