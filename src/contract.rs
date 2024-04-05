use crate::config::MyConfig;
use crate::msg::InstantiateMsg;
use crate::state::MyState;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use vaultenator::{
    admin::Administer,
    contract::Vaultenator,
    errors::ContractError,
    msg::{ExecuteMsg, MigrateMsg, QueryMsg},
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
};

pub struct MyVault;

// # Custom trait implementations
//
// - Configure implemented in src/config.rs.
// - Describe implemented in src/describe.rs
// - ManageState implemented in src/state.rs.
// - Handle implemented in src/handle.rs.

// Default implementations taken from Vaultenator crate
impl Own for MyVault {}
impl Administer<MyState> for MyVault {}
impl Query<MyConfig, MyState> for MyVault {}
impl ReplyHandler<MyConfig> for MyVault {}
impl Vaultenator<MyConfig, MyState> for MyVault {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    MyVault.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    MyVault.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    MyVault.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    MyVault.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    MyVault.migrate(deps, env, msg)
}
