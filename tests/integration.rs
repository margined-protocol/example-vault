mod helpers;
extern crate margined_vault;
use cw_vault_standard::{VaultStandardInfoResponse, VaultStandardQueryMsg};
use helpers::setup::{ContractTester, TestEnv};
use margined_vault::msg::{ExtensionQueryMsg, MarginedExtensionQueryMsg, QueryMsg};
use margined_vault::structs::{ConfigResponse, StateResponse};
use osmosis_test_tube::{Module, Wasm};

#[test]
fn query_standard_info() {
    let mut test_env = TestEnv::new();

    match test_env.initialize() {
        Ok(contract_info) => {
            let wasm = Wasm::new(&test_env.app);
            let vault_info = wasm
                .query::<QueryMsg, VaultStandardInfoResponse>(
                    &contract_info.addr,
                    &VaultStandardQueryMsg::VaultStandardInfo {},
                )
                .unwrap();
            assert_eq!(vault_info.version, 1);
            assert_eq!(vault_info.extensions.len(), 2);
            let expected_extensions = vec!["lockup".to_string(), "force-unlock".to_string()];
            assert_eq!(vault_info.extensions, expected_extensions);
        }
        Err(e) => {
            println!("Test Environment initialization failed: {}", e);
        }
    }
}

#[test]
fn query_config() {
    let mut test_env = TestEnv::new();

    match test_env.initialize() {
        Ok(contract_info) => {
            let wasm = Wasm::new(&test_env.app);

            let contract_addr = contract_info.addr;

            let config: ConfigResponse = wasm
                .query(
                    &contract_addr,
                    &QueryMsg::VaultExtension(ExtensionQueryMsg::Margined(
                        MarginedExtensionQueryMsg::Config {},
                    )),
                )
                .unwrap();

            let expected_strategy_denom =
                format!("factory/{}/{}", contract_addr, env!("CARGO_PKG_NAME"));

            assert_eq!(config.strategy_denom, expected_strategy_denom);
            assert_eq!(config.base_denom, "uosmo".to_string());
        }
        Err(e) => {
            println!("Test Environment initialization failed: {}", e);
        }
    }
}

#[test]
fn query_state() {
    let mut test_env = TestEnv::new();

    match test_env.initialize() {
        Ok(contract_info) => {
            let wasm = Wasm::new(&test_env.app);

            let contract_addr = contract_info.addr;

            let state: StateResponse = wasm
                .query(
                    &contract_addr,
                    &QueryMsg::VaultExtension(ExtensionQueryMsg::Margined(
                        MarginedExtensionQueryMsg::State {},
                    )),
                )
                .unwrap();

            let timestamp = &test_env.app.get_block_timestamp();

            assert_eq!(state.is_open, false);
            assert_eq!(state.is_paused, true);
            assert_eq!(state.last_pause, *timestamp);
        }
        Err(e) => {
            println!("Test Environment initialization failed: {}", e);
        }
    }
}
