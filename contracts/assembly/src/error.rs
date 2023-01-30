use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("No voting power")]
    NoVotePower {},

    #[error("Vote option is invalid")]
    InvalidVote {},

    #[error("This dear leader is result of your imagination")]
    InvalidDearLeader {},

    #[error("This dear leader already is your dear leader")]
    AlreadyYourDearLeader {},

    #[error("This dear leader is result of your imagination")]
    DearLeaderNotRegistered {},

    #[error("The address sent is invalid")]
    InvalidAddr {},

    #[error("This address is already registered as a wanna be")]
    AlreadyIsADearLeader {},

    #[error("User account not registered")]
    AccountNotRegistered {},

    #[error("Proposal was not registered yet")]
    ProposalNotRegistered {},

    #[error("The vote power for this user account is already registered")]
    UserAccountAlreadyRegister {},

    #[error("There is a bug somehere")]
    InternalErrorInLogic {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
