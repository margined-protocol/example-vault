use crate::ownership::OwnerProposal;
use crate::structs::{Config, State};
use cosmwasm_std::{ensure, StdError, StdResult};
use cw_controllers::Admin;
use cw_storage_plus::Item;

pub const DEFAULT_STRATEGY_CAP: u128 = 10_000_000_000_000_000_000_000u128;
pub const OWNER: Admin = Admin::new("owner");
pub const STATE: Item<State> = Item::new("state");
pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl State {
    pub fn is_open_and_unpaused(&self) -> StdResult<()> {
        ensure!(
            self.is_open,
            StdError::generic_err("Cannot perform action as contract is not open")
        );

        ensure!(
            !self.is_paused,
            StdError::generic_err("Cannot perform action as contract is paused")
        );

        Ok(())
    }
}
