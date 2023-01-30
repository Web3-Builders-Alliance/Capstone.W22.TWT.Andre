use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    uninmp: String,
}

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
    Withdraw {
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
