use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateUserAccount {},
    CreateDearLeaderAccount { dear_leader_addr: String },
    SetUserAccountsCodeId { user_accounts_code_id: u64 },
    SetDearLeaderAccountsCodeId { dear_leader_accounts_code_id: u64 },
    SetAssemblyAddr { assembly_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // returns the list of user accounts
    #[returns(GetUserAccountListResponse)]
    GetUserAccountList {},

    // returns the list of owners
    #[returns(GetOwnsersListResponse)]
    GetOwnersList {},

    // given the owner address, returns the user_accounnt address
    #[returns(GetUserAccountAddrResponse)]
    GetUserAccountAddr { owner_addr: String },

    // given the owner address, returns the dear_leader_account address
    #[returns(GetDearLeaderAccountResponse)]
    GetDearLeaderAccountAddr { owner_addr: String },

    // given the owner address, returns the dear_leader_account address
    #[returns(GetAminAddrResponse)]
    GetAdminAddr {},

    // return user_accounts_code_id, dear_leader_accounts_code_id and assembly_addr
    #[returns(GetConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct GetUserAccountListResponse {
    pub user_account_list: Vec<String>,
}

#[cw_serde]
pub struct GetOwnsersListResponse {
    pub owners_list: Vec<String>,
}

#[cw_serde]
pub struct GetUserAccountAddrResponse {
    pub user_account_addr: String,
}

#[cw_serde]
pub struct GetDearLeaderAccountResponse {
    pub dear_leader_account_addr: String,
}

#[cw_serde]
pub struct GetAminAddrResponse {
    pub admin_addr: String,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub admin_addr: String,
    pub user_accounts_code_id: u64,
    pub dear_leader_accounts_code_id: u64,
    pub assembly_addr: String,
}
