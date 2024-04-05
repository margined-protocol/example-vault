use crate::msg::InstantiateMsg;
use cosmwasm_schema::cw_serde;
use serde::{de::DeserializeOwned, Serialize};

use vaultenator::config::Configure;
use vaultenator::errors::ContractError;

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

    fn init_config<M>(deps: &mut DepsMut, msg: &M) -> Result<Self, ContractError>
    where
        M: Serialize + DeserializeOwned,
    {
        let instantiate_msg: InstantiateMsg = serde_json::from_slice(
            &serde_json::to_vec(msg).map_err(|_| ContractError::InvalidMessage {})?,
        )
        .map_err(|_| ContractError::InvalidMessage {})?;

        let config = Self {
            // Initialize fields from `instantiate_msg`
            strategy_cap: Uint128::zero(),
            strategy_denom: None,
            base_denom: instantiate_msg.base_denom.clone(),
            test: "hello".to_string(),
        };

        config.save_to_storage(deps)?;

        Ok(config)
    }
}
