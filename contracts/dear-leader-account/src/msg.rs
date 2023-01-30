use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub assembly_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Vote { proposal_id: u64, vote_option: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOwnerAddrResponse)]
    GetOwnerAddr {},
}

#[cw_serde]
pub struct GetOwnerAddrResponse {
    pub owner_addr: String,
}
