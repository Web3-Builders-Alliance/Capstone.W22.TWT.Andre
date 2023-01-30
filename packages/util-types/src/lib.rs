use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use thiserror::Error;

#[cw_serde]
pub enum ExecuteMsg {
    Delegate {
        validator_addr: String,
    },
    Undelegate {
        amount: Uint128,
        validator_addr: String,
    },
    Claim {},
    Redelagate {
        from_validator_addr: String,
        to_validator_addr: String,
        amount: Uint128,
    },
    UndelegateAll {},
    TransferVotePower {
        dear_leader_addr: String,
    },
    AssemblyVote {
        proposal_id: u64,
        vote_option: u64,
    },
    Vote {
        proposal_id: u64,
        vote_option: u64,
    },
    DearLeaderVote {
        proposal_id: u64,
        vote_option: u64,
    },
    RegisterDearLeader {
        new_dear_leader_addr: String,
    },
    RegisterUserAccount {},
    UnregisterUserAccount {},
    UserAccountVote {
        proposal_id: u64,
        vote_option: u64,
    },
}

#[cw_serde]
pub enum InstantiateMsg {
    InstantiateUserAccountMsg {},
    InstatiateDearLeaderAccount {},
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("There is a bug somehere")]
    InternalErrorInLogicCommon {},

    #[error("Reply id not recognized")]
    UnknownReplyIdCommon {},
}
