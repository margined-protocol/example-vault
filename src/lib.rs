use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use margined_vault::errors::ContractError;
use margined_vault::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use margined_vault::{Describe, MarginedVault};

pub struct ExampleVault;

impl Describe for ExampleVault {
    const CONTRACT_NAME: &'static str = env!("CARGO_PKG_NAME");
    const VAULT_STANDARD_VERSION: u16 = 1;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2] = ["lockup", "force-unlock"];
}

impl MarginedVault for ExampleVault {
    // Implement any of the functions in MarginedVault here
    // Or fallback to the implementations in MarginedVault if not specified
    // It is envisaged we might implement Deposit and Redeem
    // TODO work out how to make Config and State objects dynamic
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ExampleVault.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    ExampleVault.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    ExampleVault.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    ExampleVault.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    ExampleVault.migrate(deps, env, msg)
}
