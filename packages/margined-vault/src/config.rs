use crate::errors::ContractError;
use crate::msg::InstantiateMsg;
use cw_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{Deps, DepsMut};

pub trait Configure: Serialize + DeserializeOwned + Sized {
    const CONFIG_KEY: &'static str;

    fn init_config(deps: &mut DepsMut, msg: &InstantiateMsg) -> Result<Self, ContractError>
    where
        Self: Sized;

    fn update_strategy_denom(&mut self, denom: String);

    fn save_to_storage(&self, deps: &mut DepsMut) -> Result<(), ContractError> {
        let config_item = Item::<Self>::new(Self::CONFIG_KEY);
        config_item
            .save(deps.storage, self)
            .map_err(ContractError::from)
    }

    fn get_from_storage(deps: Deps) -> Result<Self, ContractError> {
        let config_item = Item::<Self>::new(Self::CONFIG_KEY);
        config_item.load(deps.storage).map_err(ContractError::from)
    }
}
