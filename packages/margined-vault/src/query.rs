use crate::state::{CONFIG, STATE};
use crate::structs::{ConfigResponse, StateResponse};
use cosmwasm_std::{Deps, StdResult};
use cw2::get_contract_version;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage).unwrap();

    let contract = get_contract_version(deps.storage)?;

    Ok(ConfigResponse {
        strategy_cap: config.strategy_cap,
        strategy_denom: config.strategy_denom.unwrap_or_default(),
        base_denom: config.base_denom,
        version: contract.version,
    })
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage).unwrap();

    Ok(StateResponse {
        is_open: state.is_open,
        is_paused: state.is_paused,
        last_pause: state.last_pause,
    })
}
