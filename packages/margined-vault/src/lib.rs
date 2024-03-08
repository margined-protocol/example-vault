pub mod errors;
mod handlers;
mod helpers;
pub mod msg;
mod ownership;
mod query;
mod reply;
mod state;
pub mod structs;

use crate::errors::ContractError;
use crate::handlers::{handle_open_contract, handle_pause, handle_unpause};
use crate::helpers::{initialise_config, initialise_state};
use crate::msg::{
    create_denom_message, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, InstantiateMsg,
    MarginedExtensionExecuteMsg, MarginedExtensionQueryMsg, MigrateMsg, QueryMsg,
};
use crate::ownership::{
    handle_claim_ownership, handle_ownership_proposal, handle_ownership_proposal_rejection,
};
use crate::query::{query_config, query_state};
use crate::reply::handle_create_strategy_denom_reply;
use crate::state::{CONTRACT_VERSION, OWNER, OWNERSHIP_PROPOSAL};

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_vault_standard::msg::VaultStandardInfoResponse;

pub const CREATE_STRATEGY_DENOM_REPLY_ID: u64 = 1u64;

pub trait Describe {
    const CONTRACT_NAME: &'static str;
    const VAULT_STANDARD_VERSION: u16;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2];
}

pub trait MarginedVault: Describe {
    fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        self.handle_instantiate(deps, env, info, msg)
    }

    fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Deposit {
                amount: _,
                recipient: _,
            } => {
                unimplemented!("not implemented")
            }
            ExecuteMsg::Redeem {
                recipient: _,
                amount: _,
            } => {
                unimplemented!("not implemented")
            }
            ExecuteMsg::VaultExtension(msg) => match msg {
                ExtensionExecuteMsg::Margined(msg) => match msg {
                    MarginedExtensionExecuteMsg::ClaimOwnership {} => {
                        handle_claim_ownership(deps, info, env, OWNER, OWNERSHIP_PROPOSAL)
                    }
                    MarginedExtensionExecuteMsg::UnPause {} => handle_unpause(deps, info),
                    MarginedExtensionExecuteMsg::SetOpen {} => {
                        handle_open_contract(deps, env, info)
                    }
                    MarginedExtensionExecuteMsg::UpdateConfig {} => {
                        unimplemented!("not implemented")
                        //handle_update_config(deps, info, new_config)
                    }
                    MarginedExtensionExecuteMsg::RejectOwner {} => {
                        handle_ownership_proposal_rejection(deps, info, OWNER, OWNERSHIP_PROPOSAL)
                    }
                    MarginedExtensionExecuteMsg::Pause {} => handle_pause(deps, info),
                    MarginedExtensionExecuteMsg::ProposeNewOwner {
                        new_owner,
                        duration,
                    } => handle_ownership_proposal(
                        deps,
                        info,
                        env,
                        new_owner,
                        duration,
                        OWNER,
                        OWNERSHIP_PROPOSAL,
                    ),
                },
            },
        }
    }

    fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::VaultStandardInfo {} => to_json_binary(&VaultStandardInfoResponse {
                version: Self::VAULT_STANDARD_VERSION,
                extensions: Self::VAULT_STANDARD_EXTENSIONS
                    .iter()
                    .map(|&s| s.into())
                    .collect(),
            }),
            QueryMsg::Info {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::PreviewDeposit { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::PreviewRedeem { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::TotalAssets {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::TotalVaultTokenSupply {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::ConvertToShares { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::ConvertToAssets { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::VaultExtension(msg) => match msg {
                ExtensionQueryMsg::Margined(msg) => match msg {
                    MarginedExtensionQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
                    MarginedExtensionQueryMsg::State {} => to_json_binary(&query_state(deps)?),
                },
            },
        }
    }

    fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            CREATE_STRATEGY_DENOM_REPLY_ID => handle_create_strategy_denom_reply(deps, env, msg),
            _ => Err(ContractError::InvalidReplyId),
        }
    }

    fn migrate(&self, deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
        unimplemented!()
    }
    // Define handle_instantiate function
    fn handle_instantiate(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(
            deps.storage,
            format!("crates.io:{}", Self::CONTRACT_NAME),
            CONTRACT_VERSION,
        )?;

        initialise_state(&mut deps, &env)?;
        initialise_config(&mut deps, &msg)?;

        OWNER.set(deps, Some(info.sender))?;

        let create_denom_sub_msg = SubMsg::reply_always(
            create_denom_message(&env.contract.address, Self::CONTRACT_NAME.to_string()),
            CREATE_STRATEGY_DENOM_REPLY_ID,
        );

        Ok(Response::new()
            .add_submessages([create_denom_sub_msg])
            .add_attribute("action", "instantiate"))
    }
}
