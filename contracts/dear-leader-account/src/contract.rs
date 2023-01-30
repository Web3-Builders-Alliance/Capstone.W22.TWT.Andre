#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, ASSEMBLY_ADDR, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dear_leader_acount";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.to_string(),
    };
    CONFIG.save(deps.storage, &config)?;

    ASSEMBLY_ADDR.save(deps.storage, &msg.assembly_addr)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate_dear_leader_account")
        .add_attribute("owner", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Vote {
            proposal_id,
            vote_option,
        } => execute::vote(deps, env, info, proposal_id, vote_option),
    }
}

pub mod execute {
    use cosmwasm_std::WasmMsg;

    use crate::state::{ASSEMBLY_ADDR, CONFIG};
    use util_types::ExecuteMsg as ExecuteMsgCommon;

    use super::*;

    pub fn vote(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote_option: u64,
    ) -> Result<Response, ContractError> {
        // validate contract caller is owner
        validate_owner(deps.as_ref(), &info)?;
        // create vote message to assembly contract
        let msg = WasmMsg::Execute {
            contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
            msg: to_binary(&ExecuteMsgCommon::DearLeaderVote {
                proposal_id,
                vote_option,
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_message(msg))
    }

    pub fn validate_owner(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
        let config = CONFIG.load(deps.storage)?;
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwnerAddr {} => to_binary(&query::get_owner_addr(deps)?),
    }
}

pub mod query {

    use crate::msg::GetOwnerAddrResponse;

    use super::*;

    pub fn get_owner_addr(deps: Deps) -> StdResult<Binary> {
        let owner_addr = CONFIG.load(deps.storage)?.owner;
        let response = GetOwnerAddrResponse { owner_addr };
        to_binary(&response)
    }
}

#[cfg(test)]
mod tests {}
