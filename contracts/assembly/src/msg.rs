use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UserAccountVote { proposal_id: u64, vote_option: u64 },
    DearLeaderVote { proposal_id: u64, vote_option: u64 },
    TransferVotePower { dear_leader_addr: String },
    ReclaimVotePower {},
    RegisterDearLeader { new_dear_leader_addr: String },
    RegisterUserAccount {},
    UnregisterUserAccount {},
    SetAccountFactoryAddr { account_factory_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // returns the list of user accounts delegated to the dear leader
    #[returns(GetDearLeaderDelegatoresResponse)]
    GetDearLeaderDelegatores { dear_leader_addr: String },

    // returns the dear leader addr to whom the user account is delegated
    #[returns(GetUserAccountLeaderResponse)]
    GetUserAccountLeader { user_account_addr: String },

    // returns the dear leader addr to whom the user account is delegated
    #[returns(GetVoteProposalByUserAndPropResponse)]
    GetVoteProposalByUserAndProp {
        user_account_addr: String,
        proposal_id: u64,
    },

    // return user_accounts_code_id, dear_leader_accounts_code_id and assembly_addr
    #[returns(GetConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct GetDearLeaderDelegatoresResponse {
    pub delegatores_list: Option<Vec<String>>,
}

#[cw_serde]
pub struct GetUserAccountLeaderResponse {
    pub dear_leader_addr: Option<String>,
}

#[cw_serde]
pub struct GetVoteProposalByUserAndPropResponse {
    pub voted: bool,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub admin_addr: String,
    pub accounts_factory_addr: String,
}
