use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, Timestamp};
use vaultenator::{errors::ContractError, state::ManageState};

#[cw_serde]
pub struct MyState {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}

impl ManageState for MyState {
    const STATE_KEY: &'static str = "state";

    fn is_contract_open(deps: Deps) -> Result<bool, ContractError> {
        let state = Self::get_from_storage(deps)?;
        Ok(state.is_open)
    }

    fn is_contract_paused(deps: Deps) -> Result<bool, ContractError> {
        let state = Self::get_from_storage(deps)?;
        Ok(state.is_paused)
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    fn init_state(deps: &mut DepsMut, env: &Env) -> Result<(), ContractError> {
        let initial_state = MyState {
            is_open: false,
            is_paused: true,
            last_pause: env.block.time,
        };
        initial_state.save_to_storage(deps)
    }

    fn update_state(&mut self, deps: &mut DepsMut) -> Result<(), ContractError> {
        self.is_paused = !self.is_paused;
        self.save_to_storage(deps)
    }
}
