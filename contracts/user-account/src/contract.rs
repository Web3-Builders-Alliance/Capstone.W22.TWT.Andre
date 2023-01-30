#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:user_acount";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const UNDELEGATION_REPLY_ID: u64 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Delegate { validator_addr } => {
            execute::delegate(deps, env, info, validator_addr)
        }
        ExecuteMsg::Undelegate {
            amount,
            validator_addr,
        } => execute::undelegate(deps, env, info, amount, validator_addr),
        ExecuteMsg::UndelegateAll {} => execute::undelegate_all(deps, env, info),
        ExecuteMsg::Claim {} => execute::claim(deps, env, info),
        ExecuteMsg::Redelagate {
            from_validator_addr,
            to_validator_addr,
            amount,
        } => execute::redelegate(
            deps,
            env,
            info,
            from_validator_addr,
            to_validator_addr,
            amount,
        ),
        ExecuteMsg::TransferVotePower { dear_leader_addr } => {
            execute::transfer_vote_power(deps, env, info, dear_leader_addr)
        }
        ExecuteMsg::AssemblyVote {
            proposal_id,
            vote_option,
        } => execute::assembly_vote(deps, env, info, proposal_id, vote_option),
        ExecuteMsg::Vote {
            proposal_id,
            vote_option,
        } => execute::vote(deps, env, info, proposal_id, vote_option),
        ExecuteMsg::Withdraw { amount } => execute::withdraw(deps, info, amount),
    }
}

pub mod execute {
    use cosmwasm_std::{
        BankMsg, Coin, DistributionMsg, GovMsg, StakingMsg, SubMsg, Uint128, VoteOption, WasmMsg,
    };
    use cw_utils::must_pay;
    use util_types::ExecuteMsg as CommonExecuteMsg;

    use crate::state::{ASSEMBLY_ADDR, BOSS_ADDR};

    use super::*;

    pub fn delegate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        validator_addr: String,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // 1 - validate that only one token is sent
        // 2 - validate that the token is the native token
        let native_denom = deps.querier.query_bonded_denom()?;
        let sent_token =
            must_pay(&info, native_denom.as_str()).map_err(|_| ContractError::WrongToken {})?;

        // validate validator_addr
        let valid_val_addr = deps
            .api
            .addr_validate(&validator_addr)
            .map_err(|_| ContractError::InvalidAddr {})?;

        // create the staking message
        let msg = StakingMsg::Delegate {
            validator: valid_val_addr.to_string(),
            amount: info.funds[0].clone(),
        };

        // check if it is first delegation
        let is_first_delegation = deps
            .querier
            .query_all_delegations(env.contract.address)?
            .is_empty();

        // if it is first delegation, create msg to register vote power in the assembly
        if is_first_delegation {
            let register_msg = WasmMsg::Execute {
                contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
                msg: to_binary(&CommonExecuteMsg::RegisterUserAccount {})?,
                funds: vec![],
            };

            Ok(Response::new()
                .add_attribute("action", "delegate")
                .add_attribute("boss", info.sender.to_string())
                .add_attribute("amount", sent_token.to_string())
                .add_attribute("to", validator_addr)
                .add_message(msg)
                .add_message(register_msg))
        } else {
            Ok(Response::new()
                .add_attribute("action", "delegate")
                .add_attribute("boss", info.sender.to_string())
                .add_attribute("amount", sent_token.to_string())
                .add_attribute("to", validator_addr)
                .add_message(msg))
        }
    }

    pub fn undelegate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: Uint128,
        validator_addr: String,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // validate validator_addr
        let valid_val_addr = deps
            .api
            .addr_validate(&validator_addr)
            .map_err(|_| ContractError::InvalidAddr {})?;

        // confirm existing delegation and amount sent is within the bounds
        let delegation = deps
            .querier
            .query_delegation(env.contract.address, valid_val_addr)?
            .ok_or(ContractError::NoDelegation {})?;

        // validate amount requested to undelegate is within the bounds
        if amount >= delegation.amount.amount {
            return Err(ContractError::UndelegateAmountTooHigh {});
        }
        // create undelegate message
        let msg = StakingMsg::Undelegate {
            validator: delegation.validator,
            amount: Coin {
                denom: delegation.amount.denom,
                amount: delegation.amount.amount,
            },
        };

        // add reply on sucess, so we can check if there is no delegation left, to unregister vote power in the assembly
        let sub_msg: SubMsg = SubMsg::reply_on_success(msg, UNDELEGATION_REPLY_ID);

        Ok(Response::new()
            .add_attribute("action", "undelegate")
            .add_attribute("boss", info.sender.to_string())
            .add_attribute("amount", amount.to_string())
            .add_attribute("from_validator", validator_addr)
            .add_submessage(sub_msg))
    }

    pub fn undelegate_all(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // get all delegations
        let delegations = deps.querier.query_all_delegations(env.contract.address)?;

        if delegations.is_empty() {
            return Err(ContractError::NoDelegation {});
        }
        // create undelegate message for each delegation
        let msgs = delegations
            .iter()
            .map(|delegation| StakingMsg::Undelegate {
                validator: delegation.validator.clone(),
                amount: Coin {
                    denom: delegation.amount.denom.clone(),
                    amount: delegation.amount.amount,
                },
            })
            .collect::<Vec<StakingMsg>>();

        // create message to unregister vote power in the assembly
        let msg = WasmMsg::Execute {
            contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
            msg: to_binary(&CommonExecuteMsg::UnregisterUserAccount {})?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "undelegate")
            .add_attribute("boss", info.sender.to_string())
            .add_messages(msgs)
            .add_message(msg))
        // .add_submessage(submessage))
    }

    pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // get all delegations
        let delegations = deps.querier.query_all_delegations(env.contract.address)?;
        // create claim rewards messages for each delegation
        let msgs = delegations
            .iter()
            .map(|delegation| DistributionMsg::WithdrawDelegatorReward {
                validator: delegation.validator.clone(),
            })
            .collect::<Vec<DistributionMsg>>();
        // return response
        Ok(Response::new()
            .add_attribute("action", "claim_rewards")
            .add_attribute("boss", info.sender.to_string())
            .add_messages(msgs))
    }

    pub fn redelegate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from_validator_addr: String,
        to_validator_addr: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // validate from_validator_addr
        let valid_from_val_addr = deps
            .api
            .addr_validate(&from_validator_addr)
            .map_err(|_| ContractError::InvalidAddr {})?;

        // validate to_validator_addr
        let valid_to_val_addr = deps
            .api
            .addr_validate(&to_validator_addr)
            .map_err(|_| ContractError::InvalidAddr {})?;
        // validate amount is delegated to from_validator_addr and is valid

        let delegation = deps
            .querier
            .query_delegation(env.contract.address, valid_from_val_addr.clone())?
            .ok_or(ContractError::NoDelegation {})?;

        // validate amount requested to undelegate is within the bounds
        if amount >= delegation.amount.amount {
            return Err(ContractError::UndelegateAmountTooHigh {});
        }

        // create restake message
        let msg = StakingMsg::Redelegate {
            src_validator: valid_from_val_addr.to_string(),
            dst_validator: valid_to_val_addr.to_string(),
            amount: Coin {
                denom: delegation.amount.denom,
                amount: delegation.amount.amount,
            },
        };

        Ok(Response::new()
            .add_attribute("action", "redelegate")
            .add_attribute("from_validator", from_validator_addr)
            .add_attribute("to_validator", to_validator_addr)
            .add_attribute("amount", amount.to_string())
            .add_message(msg))
    }

    pub fn transfer_vote_power(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        dear_leader_addr: String,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;
        // validate dear_leader_addr
        let valid_dear_leader_addr = deps
            .api
            .addr_validate(&dear_leader_addr)
            .map_err(|_| ContractError::InvalidAddr {})?;

        // NOT NEEDED -> DEAR_LEADER_ADDR.save(deps.storage, &valid_dear_leader_addr.to_string())?;

        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;

        // communicate with Voting Command Center about the news
        let msg = WasmMsg::Execute {
            contract_addr: assembly_addr,
            msg: to_binary(&ExecuteMsg::TransferVotePower {
                dear_leader_addr: valid_dear_leader_addr.to_string(),
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "transfer_vote_power")
            .add_attribute("dear_leader", valid_dear_leader_addr)
            .add_message(msg))
    }

    pub fn assembly_vote(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote: u64,
    ) -> Result<Response, ContractError> {
        // confirm assembly is calling
        validate_assembly_call(deps.as_ref(), &info)?;

        //convert vote to VoteOption
        let vote = match vote {
            1 => VoteOption::Yes,
            2 => VoteOption::No,
            3 => VoteOption::Abstain,
            4 => VoteOption::NoWithVeto,
            _ => return Err(ContractError::InvalidVote {}),
        };

        // create vote message
        let msg = GovMsg::Vote { proposal_id, vote };

        Ok(Response::new()
            .add_attribute("action", "vote")
            // .add_attribute("dear_leader_addr", dear_leader_addr)
            .add_message(msg))
    }

    pub fn vote(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote: u64,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;

        // create vote message to the Assembly
        let msg = WasmMsg::Execute {
            contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
            msg: to_binary(&CommonExecuteMsg::Vote {
                proposal_id,
                vote_option: vote,
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_message(msg))
    }

    pub fn withdraw(
        deps: DepsMut,
        info: MessageInfo,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        // confirm boss is calling
        validate_boss(deps.as_ref(), &info)?;

        // create withdraw message
        let msg = BankMsg::Send {
            to_address: BOSS_ADDR.load(deps.storage)?,
            amount: vec![Coin {
                denom: deps.querier.query_bonded_denom()?,
                amount,
            }],
        };

        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_attribute("amount", amount.to_string())
            .add_message(msg))
    }

    fn validate_boss(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
        let boss = BOSS_ADDR.load(deps.storage)?;
        if info.sender != boss {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }

    fn validate_assembly_call(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;
        if info.sender != assembly_addr {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        UNDELEGATION_REPLY_ID => reply::unregister_vote_if_last_delegation(deps, env, msg),
        _ => Err(ContractError::UnknownReplyIdCommon {}),
    }
}

pub mod reply {
    use cosmwasm_std::WasmMsg;
    use util_types::ExecuteMsg as CommonExecuteMsg;

    use crate::state::ASSEMBLY_ADDR;

    use super::*;

    pub fn unregister_vote_if_last_delegation(
        deps: DepsMut,
        env: Env,
        _msg: Reply,
    ) -> Result<Response, ContractError> {
        let delegations = deps.querier.query_all_delegations(env.contract.address)?;

        // if there are no more delegations, send message to Assembly to unregister the user account
        if delegations.is_empty() {
            // create message to the Assembly
            let msg = WasmMsg::Execute {
                contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
                msg: to_binary(&CommonExecuteMsg::UnregisterUserAccount {})?,
                funds: vec![],
            };

            Ok(Response::new()
                .add_attribute("action", "unregister_vote_on_reply")
                .add_message(msg))
        } else {
            Ok(Response::default())
        }
    }
}

#[cfg(test)]
mod tests {}
