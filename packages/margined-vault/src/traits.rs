use crate::errors::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};

pub trait MyContract {
    fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError>;
    fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
    fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary>;
    fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError>;
    fn migrate(&self, deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError>;
    fn handle_instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        msg: MigrateMsg,
    ) -> Result<Response, ContractError>;
}
