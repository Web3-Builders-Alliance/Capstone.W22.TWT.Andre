#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::ADMIN_ADDR;
use util_types::ExecuteMsg as CommonExecuteMsg;
use util_types::InstantiateMsg as CommonInstantiateMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:accounts_factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const USER_ACCOUNT_REPLY_ID: u64 = 0;
const DEAR_LEADER_ACCOUNT_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // save instantiator as admin
    ADMIN_ADDR.save(deps.storage, &info.sender.to_string())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate_factory")
        .add_attribute("done_by", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateUserAccount {} => execute::create_user_account(deps, env, info),
        ExecuteMsg::CreateDearLeaderAccount { dear_leader_addr } => {
            execute::create_dear_leader_account(deps, env, info, dear_leader_addr)
        }
        ExecuteMsg::SetUserAccountsCodeId {
            user_accounts_code_id,
        } => execute::set_user_accounts_code_id(deps, env, info, user_accounts_code_id),
        ExecuteMsg::SetDearLeaderAccountsCodeId {
            dear_leader_accounts_code_id,
        } => {
            execute::set_dear_leader_accounts_code_id(deps, env, info, dear_leader_accounts_code_id)
        }
        ExecuteMsg::SetAssemblyAddr { assembly_addr } => {
            execute::set_assembly_addr(deps, env, info, assembly_addr)
        }
    }
}

pub mod execute {
    use cosmwasm_std::{SubMsg, WasmMsg};

    use crate::state::{
        ADMIN_ADDR, ASSEMBLY_ADDR, DEAR_LEADER_ACCOUNTS_CODE_ID,
        DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT, STATE_MANAGEMENT_CACHE_WITHIN_TX,
        USER_ACCOUNTS_CODE_ID, USER_ACCOUNTS_UNDER_MANAGEMENT,
    };

    use super::*;

    pub fn create_dear_leader_account(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        dear_leader_addr: String,
    ) -> Result<Response, ContractError> {
        // this accounts can only be created by the admin of the contract
        let admin_addr = ADMIN_ADDR.load(deps.storage)?;
        if info.sender != admin_addr {
            return Err(ContractError::Unauthorized {});
        }

        // validate dear leader addr and check that the dear_leader don't have an account yet
        let validated_addr = deps.api.addr_validate(&dear_leader_addr)?;
        if DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT.has(deps.storage, validated_addr.to_string()) {
            return Err(ContractError::DearLeaderAccountAlreadyExists);
        }

        STATE_MANAGEMENT_CACHE_WITHIN_TX.save(deps.storage, &info.sender.to_string())?;

        // check if dear_leader_accounts_code_id is set
        let dear_leader_account_code_id = DEAR_LEADER_ACCOUNTS_CODE_ID
            .load(deps.storage)
            .unwrap_or_default();

        if dear_leader_account_code_id == 0 {
            return Err(ContractError::DearLeaderAccountsCodeIdNotSet);
        }

        // create msg to register dear leader in the assembly contract
        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;
        // ensure assembly addr is registered
        if assembly_addr.is_empty() {
            return Err(ContractError::AssemblyAddrNotSet);
        }

        let msg = WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: dear_leader_account_code_id,
            msg: to_binary(&CommonInstantiateMsg::InstatiateDearLeaderAccount {})?,
            funds: vec![],
            label: info.sender.to_string() + "_user_account",
        };
        let submessage: SubMsg = SubMsg::reply_on_success(msg, DEAR_LEADER_ACCOUNT_REPLY_ID);

        Ok(Response::new()
            .add_attribute("action", "create_user_account")
            .add_attribute("created_by", info.sender.to_string())
            .add_attribute("dear_leader_addr", validated_addr.to_string())
            .add_submessage(submessage))
    }

    pub fn create_user_account(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // validate that user don't have an account yet
        if USER_ACCOUNTS_UNDER_MANAGEMENT.has(deps.storage, info.sender.to_string()) {
            return Err(ContractError::UserAccountAlreadyExists);
        }

        STATE_MANAGEMENT_CACHE_WITHIN_TX.save(deps.storage, &info.sender.to_string())?;

        // create user account with instantiate msg with ReplyOn::Success
        let user_account_code_id = USER_ACCOUNTS_CODE_ID.load(deps.storage).unwrap_or_default();
        if user_account_code_id == 0 {
            return Err(ContractError::UserAccountCodeIdNotSet);
        }
        let instantiate_msg = WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: user_account_code_id,
            msg: to_binary(&CommonInstantiateMsg::InstantiateUserAccountMsg {})?,
            funds: vec![],
            label: info.sender.to_string() + "_user_account",
        };
        let submessage: SubMsg = SubMsg::reply_on_success(instantiate_msg, USER_ACCOUNT_REPLY_ID);

        Ok(Response::new()
            .add_attribute("action", "create_user_account")
            .add_attribute("created_by", info.sender.to_string())
            .add_submessage(submessage))
    }

    pub fn set_user_accounts_code_id(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        user_accounts_code_id: u64,
    ) -> Result<Response, ContractError> {
        // this type of config can only be set by the admin of the contract
        let admin_addr = ADMIN_ADDR.load(deps.storage)?;
        if info.sender != admin_addr {
            return Err(ContractError::Unauthorized {});
        }

        // save code id
        USER_ACCOUNTS_CODE_ID.save(deps.storage, &user_accounts_code_id)?;

        Ok(Response::new()
            .add_attribute("action", "set_user_accounts_code_id")
            .add_attribute("set_by", info.sender.to_string())
            .add_attribute("user_accounts_code_id", user_accounts_code_id.to_string()))
    }

    pub fn set_dear_leader_accounts_code_id(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        dear_leader_accounts_code_id: u64,
    ) -> Result<Response, ContractError> {
        // this type of config can only be set by the admin of the contract
        let admin_addr = ADMIN_ADDR.load(deps.storage)?;
        if info.sender != admin_addr {
            return Err(ContractError::Unauthorized {});
        }

        // save code id
        DEAR_LEADER_ACCOUNTS_CODE_ID.save(deps.storage, &dear_leader_accounts_code_id)?;

        Ok(Response::new()
            .add_attribute("action", "set_dear_leader_accounts_code_id")
            .add_attribute("set_by", info.sender.to_string())
            .add_attribute(
                "dear_leader_accounts_code_id",
                dear_leader_accounts_code_id.to_string(),
            ))
    }

    pub fn set_assembly_addr(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        assembly_addr: String,
    ) -> Result<Response, ContractError> {
        // this type of config can only be set by the admin of the contract
        let admin_addr = ADMIN_ADDR.load(deps.storage)?;
        if info.sender != admin_addr {
            return Err(ContractError::Unauthorized {});
        }

        // validate assembly addr
        let validated_addr = deps.api.addr_validate(&assembly_addr)?;

        // save code id
        ASSEMBLY_ADDR.save(deps.storage, &validated_addr.to_string())?;

        Ok(Response::new()
            .add_attribute("action", "set_assembly_addr")
            .add_attribute("set_by", info.sender.to_string())
            .add_attribute("assembly_addr", validated_addr.to_string()))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserAccountList {} => query::get_user_account_list(deps),
        QueryMsg::GetOwnersList {} => query::get_owners_list(deps),
        QueryMsg::GetUserAccountAddr { owner_addr } => {
            query::get_user_account_addr(deps, owner_addr)
        }
        QueryMsg::GetDearLeaderAccountAddr { owner_addr } => {
            query::get_dear_leader_account_addr(deps, owner_addr)
        }
        QueryMsg::GetAdminAddr {} => query::get_admin_addr(deps),
        QueryMsg::GetConfig {} => query::get_config(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        USER_ACCOUNT_REPLY_ID => reply::instantiate_user_account_reply(deps, msg),
        DEAR_LEADER_ACCOUNT_REPLY_ID => reply::instantiate_dear_leader_account_reply(deps, msg),
        _ => Err(ContractError::UnknownReplyIdCommon {}),
    }
}

pub mod reply {
    use crate::state::{
        ASSEMBLY_ADDR, DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT, STATE_MANAGEMENT_CACHE_WITHIN_TX,
        USER_ACCOUNTS_UNDER_MANAGEMENT,
    };

    use super::*;
    use cosmwasm_std::WasmMsg;
    use cw_utils::parse_reply_instantiate_data;
    //use util_types::ContractError as CommonContractError;

    pub fn instantiate_dear_leader_account_reply(
        deps: DepsMut,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let res = parse_reply_instantiate_data(msg)?;
        let dear_leader_account_addr = deps.api.addr_validate(&res.contract_address)?;

        // add dear leader account to the list of dear leader accounts under management
        let owner_addr = STATE_MANAGEMENT_CACHE_WITHIN_TX.load(deps.storage)?;
        DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT.save(
            deps.storage,
            owner_addr,
            &dear_leader_account_addr.to_string(),
        )?;

        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;

        let msg = WasmMsg::Execute {
            contract_addr: assembly_addr,
            msg: to_binary(&CommonExecuteMsg::RegisterDearLeader {
                new_dear_leader_addr: dear_leader_account_addr.to_string(),
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "instantiated_dear_leader_account_reply")
            .add_attribute(
                "dear_leader_account_ addr",
                dear_leader_account_addr.to_string(),
            )
            .add_message(msg))
    }

    pub fn instantiate_user_account_reply(
        deps: DepsMut,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let res = parse_reply_instantiate_data(msg)?;
        let user_account_addr = deps.api.addr_validate(&res.contract_address)?;

        // add user account to the list of user accounts under management
        let owner_addr = STATE_MANAGEMENT_CACHE_WITHIN_TX.load(deps.storage)?;
        USER_ACCOUNTS_UNDER_MANAGEMENT.save(
            deps.storage,
            owner_addr,
            &user_account_addr.to_string(),
        )?;

        Ok(Response::default()
            .add_attribute("instantiated_user_account", user_account_addr.to_string()))
    }
}

pub mod query {

    use cw_paginate::{paginate_map_keys, paginate_map_values};

    use crate::{
        msg::{GetConfigResponse, GetOwnsersListResponse, GetUserAccountListResponse},
        state::{
            ASSEMBLY_ADDR, DEAR_LEADER_ACCOUNTS_CODE_ID, DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT,
            USER_ACCOUNTS_CODE_ID, USER_ACCOUNTS_UNDER_MANAGEMENT,
        },
    };

    use super::*;

    pub fn get_user_account_list(deps: Deps) -> StdResult<Binary> {
        let user_account_list = paginate_map_values(
            deps,
            &USER_ACCOUNTS_UNDER_MANAGEMENT,
            None,
            None,
            cosmwasm_std::Order::Ascending,
        )?;

        to_binary(&GetUserAccountListResponse { user_account_list })
    }

    pub fn get_owners_list(deps: Deps) -> StdResult<Binary> {
        let owners_list = paginate_map_keys(
            deps,
            &USER_ACCOUNTS_UNDER_MANAGEMENT,
            None,
            None,
            cosmwasm_std::Order::Ascending,
        )?;

        to_binary(&GetOwnsersListResponse { owners_list })
    }

    pub fn get_user_account_addr(deps: Deps, owner_addr: String) -> StdResult<Binary> {
        let user_account_addr = USER_ACCOUNTS_UNDER_MANAGEMENT
            .may_load(deps.storage, owner_addr)
            .map_err(|_| ContractError::UserAccountAddrNotFound {});

        to_binary(&user_account_addr.unwrap().unwrap())
    }

    pub fn get_dear_leader_account_addr(deps: Deps, owner_addr: String) -> StdResult<Binary> {
        let dear_leader_account_addr = DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT
            .may_load(deps.storage, owner_addr)
            .map_err(|_| ContractError::DearLeaderAccountAddrNotFound {});

        to_binary(&dear_leader_account_addr.unwrap().unwrap())
    }

    pub fn get_admin_addr(deps: Deps) -> StdResult<Binary> {
        let admin_addr = ADMIN_ADDR.load(deps.storage)?;

        to_binary(&admin_addr)
    }

    pub fn get_config(deps: Deps) -> StdResult<Binary> {
        let admin_addr = ADMIN_ADDR.load(deps.storage).unwrap_or_default();
        let user_accounts_code_id = USER_ACCOUNTS_CODE_ID.load(deps.storage).unwrap_or_default();
        let dear_leader_accounts_code_id = DEAR_LEADER_ACCOUNTS_CODE_ID
            .load(deps.storage)
            .unwrap_or_default();
        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage).unwrap_or_default();

        let resp = GetConfigResponse {
            admin_addr,
            user_accounts_code_id,
            dear_leader_accounts_code_id,
            assembly_addr,
        };

        to_binary(&resp)
    }
}
