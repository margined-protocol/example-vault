use crate::errors::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{CONFIG, DEFAULT_STRATEGY_CAP, STATE};
use crate::structs::{Config, State};
use cosmwasm_std::{DepsMut, Env, StdResult, Uint128};
pub fn initialise_state(deps: &mut DepsMut, env: &Env) -> StdResult<()> {
    let state = State {
        is_open: false,
        is_paused: true,
        last_pause: env.block.time,
    };

    STATE.save(deps.storage, &state)
}

pub fn initialise_config(
    deps: &mut DepsMut,
    msg: &InstantiateMsg,
) -> Result<Config, ContractError> {
    let config = Config {
        strategy_cap: Uint128::new(DEFAULT_STRATEGY_CAP),
        strategy_denom: None,
        base_denom: msg.base_denom.clone(),
    };

    // config.validate()?;

    CONFIG.save(deps.storage, &config)?;

    Ok(config)
}
