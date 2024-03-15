use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response, StdResult,
    Timestamp, Uint128,
};
use vaultenator::{
    admin::Administer,
    config::Configure,
    contract::{Describe, Vaultenator},
    errors::ContractError,
    handlers::Handle,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
    state::{ManageState, OWNER},
};

// Define a struct for the vault
pub struct MyVault;

// Define a config object for the vault
#[cw_serde]
pub struct MyConfig {
    pub strategy_cap: Uint128,
    pub strategy_denom: Option<String>,
    pub base_denom: String,
    pub test: String,
}

// Define a state object for the vault
#[cw_serde]
pub struct MyState {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}

// Use the default implementations for Own, Administer, Query and ReplyHandler
// You may provide your own implementations of these.
impl Own for MyVault {}
impl Administer<MyState> for MyVault {}
impl Query<MyConfig, MyState> for MyVault {}
impl ReplyHandler<MyConfig> for MyVault {}

// Implement Describe for the vault
// This is used by the CW4626 standard
impl Describe for MyVault {
    const CONTRACT_NAME: &'static str = env!("CARGO_PKG_NAME");
    const VAULT_STANDARD_VERSION: u16 = 1;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2] = ["lockup", "force-unlock"];
}

// Implement Configure for the vault
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

// Implement ManageState for the vault
impl ManageState for MyState {
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

// Implment Handle
impl Handle<MyConfig, MyState> for MyVault {
    fn handle_update_config(
        &self,
        mut deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let config = MyConfig::get_from_storage(deps.as_ref())?;
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

// Use the default Vaultenator implementation to inherit the standard interface
impl Vaultenator<MyConfig, MyState> for MyVault {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    MyVault.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    MyVault.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    MyVault.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    MyVault.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    MyVault.migrate(deps, env, msg)
}
