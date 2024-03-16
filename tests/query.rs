mod helpers;
extern crate example_vault;
use example_vault::MyConfig;
use example_vault::MyState;
use helpers::setup::TestEnv;
use osmosis_test_tube::{Module, Wasm};
use vaultenator::msg::{ExtensionQueryMsg, MarginedExtensionQueryMsg, QueryMsg};

#[test]
fn query_config() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let config = wasm
        .query::<QueryMsg, MyConfig>(
            &contract_addr,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Margined(
                MarginedExtensionQueryMsg::Config {},
            )),
        )
        .unwrap();

    let expected_strategy_denom = format!("factory/{}/{}", contract_addr, env!("CARGO_PKG_NAME"));

    assert_eq!(config.strategy_denom, Some(expected_strategy_denom));
    assert_eq!(config.base_denom, "uosmo".to_string());
    assert_eq!(config.test, "hello".to_string());
}

#[test]
fn query_state() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let state: MyState = wasm
        .query(
            &contract_addr,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Margined(
                MarginedExtensionQueryMsg::State {},
            )),
        )
        .unwrap();

    let timestamp = &env.app.get_block_timestamp();

    assert_eq!(state.is_open, false);
    assert_eq!(state.is_paused, true);
    assert_eq!(state.last_pause, *timestamp);
}
