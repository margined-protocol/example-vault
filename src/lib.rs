//pub mod describe;
//use crate::describe::Describe;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response, StdResult,
    Timestamp, Uint128,
};
use margined_vault::{
    admin::Administer,
    config::Configure,
    contract::{Describe, MarginedVault},
    errors::ContractError,
    handlers::Handle,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
    state::{ManageState, OWNER},
};

pub struct ExampleVault;

impl Describe for ExampleVault {
    const CONTRACT_NAME: &'static str = env!("CARGO_PKG_NAME");
    const VAULT_STANDARD_VERSION: u16 = 1;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2] = ["lockup", "force-unlock"];
}

#[cw_serde]
pub struct ExampleConfig {
    pub strategy_cap: Uint128,
    pub strategy_denom: Option<String>,
    pub base_denom: String,
    pub test: String,
}

#[cw_serde]
pub struct ExampleState {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}

// impl Describe for ExampleVault {
//     const CONTRACT_NAME: &'static str = env!("CARGO_PKG_NAME");
//     const VAULT_STANDARD_VERSION: u16 = 1;
//     const VAULT_STANDARD_EXTENSIONS: [&'static str; 2] = ["lockup", "force-unlock"];
// }

impl Configure for ExampleConfig {
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

impl ManageState for ExampleState {
    const STATE_KEY: &'static str = "state";

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    fn init_state(deps: &mut DepsMut, env: &Env) -> Result<(), ContractError> {
        let initial_state = ExampleState {
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

impl Own for ExampleVault {}
impl Administer<ExampleState> for ExampleVault {}
impl Query<ExampleConfig, ExampleState> for ExampleVault {}
impl ReplyHandler<ExampleConfig> for ExampleVault {}

impl Handle<ExampleConfig, ExampleState> for ExampleVault {
    fn handle_update_config(
        &self,
        mut deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let config = ExampleConfig::get_from_storage(deps.as_ref())?;
        // config.strategy_cap = new_config.strategy_cap;

        config.save_to_storage(&mut deps)?;

        Ok(Response::new().add_event(Event::new("update_config")))
    }
    fn handle_deposit(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
    fn handle_redeem(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
}

impl MarginedVault<ExampleConfig, ExampleState> for ExampleVault {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ExampleVault.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    ExampleVault.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    ExampleVault.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    ExampleVault.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    ExampleVault.migrate(deps, env, msg)
}
