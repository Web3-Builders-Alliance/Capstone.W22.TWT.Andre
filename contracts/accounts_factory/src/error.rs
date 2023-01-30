use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    ParseReplyError(#[from] ParseReplyError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("User Account already exists")]
    UserAccountAlreadyExists,

    #[error("Dear Leader Account already exists")]
    DearLeaderAccountAlreadyExists,

    #[error("Reply ID not recognized")]
    UnknownReplyIdCommon {},

    #[error("User Account not found")]
    UserAccountAddrNotFound {},

    #[error("Dear Leader Account not found")]
    DearLeaderAccountAddrNotFound,

    #[error("User account code ID not set")]
    UserAccountCodeIdNotSet,

    #[error("Dear leader account code ID not set")]
    DearLeaderAccountsCodeIdNotSet,

    #[error("Assembly addr not set")]
    AssemblyAddrNotSet,
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
