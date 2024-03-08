use crate::errors::ContractError;
// use crate::maths::{calc_base_to_withdraw, calc_strategy_share};
// use crate::queries::get_total_supply;
use crate::state::{CONFIG, OWNER, STATE};
use crate::structs::{Config, State};
use cosmwasm_std::{ensure, DepsMut, Env, Event, MessageInfo, Response, StdResult, Uint128};
use cw_utils::must_pay;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmosisCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
};
pub fn handle_open_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    OWNER.assert_admin(deps.as_ref(), &info.sender)?;

    let state: State = STATE.load(deps.storage)?;
    ensure!(!state.is_open, ContractError::IsOpen {});

    // set the contract to open
    STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.is_open = true;
        state.is_paused = false;
        Ok(state)
    })?;

    Ok(Response::new().add_event(Event::new("open_contract")))
}

pub fn handle_pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    ensure!(
        OWNER.is_admin(deps.as_ref(), &info.sender)?,
        ContractError::Unauthorized {}
    );

    if !state.is_open {
        return Err(ContractError::Paused {});
    }
    state.is_open = false;

    STATE.save(deps.storage, &state)?;

    Ok(Response::default().add_event(Event::new("paused")))
}

pub fn handle_unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    ensure!(
        OWNER.is_admin(deps.as_ref(), &info.sender)?,
        ContractError::Unauthorized {}
    );

    if state.is_open {
        return Err(ContractError::NotPaused {});
    }

    state.is_open = true;

    STATE.save(deps.storage, &state)?;

    Ok(Response::default().add_event(Event::new("unpaused")))
}

pub fn handle_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    STATE.load(deps.storage)?.is_open_and_unpaused()?;
    let config = CONFIG.load(deps.storage)?;

    let amount = must_pay(&info, &config.base_denom).map_err(|_| ContractError::InvalidFunds {})?;
    let mint_strategy_token_msg =
        mint_strategy_token(&deps, &env, info.sender.to_string(), amount)?;

    Ok(Response::default().add_message(mint_strategy_token_msg))
}

pub fn handle_redeem(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let strategy_token_denom = config
        .strategy_denom
        .ok_or(ContractError::DenomNotInitialized {})?;

    let amount =
        must_pay(&info, &strategy_token_denom).map_err(|_| ContractError::InvalidFunds {})?;

    let burn_strategy_token_msg = burn_strategy_token(&deps, &env, amount)?;

    // // Generate the burn power perp submessage
    // let burn_power_perp_submsg = withdraw(
    //     deps,
    //     &env,
    //     info.sender,
    //     strategy_token_amount,
    //     power_needed_to_repay_amount,
    //     false,
    // )?;

    // Build the response
    Ok(Response::default().add_message(burn_strategy_token_msg))
}

pub fn mint_strategy_token(
    deps: &DepsMut,
    env: &Env,
    to: String,
    amount: Uint128,
) -> Result<MsgMint, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let strategy_denom = config
        .strategy_denom
        .as_ref()
        .ok_or(ContractError::DenomNotInitialized {})?;
    let msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(OsmosisCoin {
            denom: strategy_denom.to_string(),
            amount: amount.to_string(),
        }),
        mint_to_address: to,
    };
    Ok(msg)
}

pub fn burn_strategy_token(
    deps: &DepsMut,
    env: &Env,
    amount: Uint128,
) -> Result<MsgBurn, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let strategy_denom = config
        .strategy_denom
        .as_ref()
        .ok_or(ContractError::DenomNotInitialized {})?;
    let msg = MsgBurn {
        sender: env.contract.address.to_string(),
        amount: Some(OsmosisCoin {
            denom: strategy_denom.to_string(),
            amount: amount.to_string(),
        }),
        burn_from_address: env.contract.address.to_string(),
    };
    Ok(msg)
}

// fn check_strategy_cap(
//     deps: &DepsMut,
//     deposit_amount: Uint128,
//     strategy_collateral: Uint128,
// ) -> Result<bool, ContractError> {
//     let config: Config = CONFIG.load(deps.storage)?;

//     if strategy_collateral + deposit_amount > config.strategy_cap {
//         return Err(ContractError::StrategyCapExceeded {});
//     }
//     Ok(true)
// }

// pub fn withdraw(
//     deps: DepsMut,
//     _env: &Env,
//     withdrawer: Addr,
//     strategy_denom_amount: Uint128,
// ) -> Result<bool, ContractError> {
//     let config: Config = CONFIG.load(deps.storage)?;

//     // Get the total supply of the strategy token
//     let strategy_denom = config
//         .strategy_denom
//         .as_ref()
//         .ok_or(ContractError::DenomNotInitialized {})?;
//     let strategy_denom_total_supply = get_total_supply(&deps, strategy_denom)?;

//     // Calculate the strategy share for the provided amount of strategy tokens
//     let strategy_share = calc_strategy_share(strategy_denom_amount, strategy_denom_total_supply)?;

//     // Calc total_base in some way

//     // Compute the amount of collateral to redeem
//     // let base_to_withdraw = calc_base_to_withdraw(strategy_share, total_base)?;

//     Ok(true)
// }
