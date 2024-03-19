use crate::config::MyConfig;
use crate::contract::MyVault;
use crate::state::MyState;
use cosmwasm_std::{DepsMut, Env, Event, MessageInfo, Response};
use vaultenator::errors::ContractError;
use vaultenator::handlers::Handle;
use vaultenator::state::OWNER;

impl Handle<MyConfig, MyState> for MyVault {
    fn handle_update_config(
        &self,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        // let config = MyConfig::get_from_storage(deps.as_ref())?;
        // config.strategy_cap = new_config.strategy_cap;

        // config.save_to_storage(&mut deps)?;

        Ok(Response::new().add_event(Event::new("update_config")))
    }
    fn handle_deposit(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
    fn handle_redeem(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
}
