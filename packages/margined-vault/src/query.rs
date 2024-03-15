use crate::config::Configure;
use crate::state::ManageState;
use cosmwasm_std::{to_json_binary, Binary, Deps, StdError, StdResult};
use serde::Serialize;

pub trait Query<C, S>
where
    C: Configure + Serialize,
    S: ManageState + Serialize,
{
    fn query_config(deps: Deps) -> StdResult<Binary> {
        match C::get_from_storage(deps) {
            Ok(config) => to_json_binary(&config),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }

    fn query_state(deps: Deps) -> StdResult<Binary> {
        match S::get_from_storage(deps) {
            Ok(config) => to_json_binary(&config),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }
}
