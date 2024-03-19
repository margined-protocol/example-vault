use cosmwasm_schema::cw_serde;
use vaultenator::config::Configure;
use vaultenator::errors::ContractError;
use vaultenator::msg::InstantiateMsg;

use cosmwasm_std::{DepsMut, Uint128};

#[cw_serde]
pub struct MyConfig {
    pub strategy_cap: Uint128,
    pub strategy_denom: Option<String>,
    pub base_denom: String,
    pub test: String,
}

impl Configure for MyConfig {
    const CONFIG_KEY: &'static str = "config";

    fn update_strategy_denom(&mut self, denom: String) {
        self.strategy_denom = Some(denom);
    }

    fn init_config(deps: &mut DepsMut, msg: &InstantiateMsg) -> Result<Self, ContractError> {
        let config = Self {
            strategy_cap: Uint128::zero(),
            strategy_denom: None,
            base_denom: msg.base_denom.clone(),
            test: "hello".to_string(),
        };

        Self::save_to_storage(&config, deps)?;
        Ok(config)
    }
}
