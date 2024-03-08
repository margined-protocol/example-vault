use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};

#[cw_serde]
pub struct Config {
    pub strategy_cap: Uint128,
    pub strategy_denom: Option<String>,
    pub base_denom: String,
}

#[cw_serde]
pub struct State {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}

#[cw_serde]
pub struct ConfigResponse {
    pub strategy_cap: Uint128,
    pub strategy_denom: String,
    pub base_denom: String,
    pub version: String,
}

#[cw_serde]
pub struct StateResponse {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}
