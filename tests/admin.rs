mod helpers;
extern crate example_vault;
use cw_controllers::AdminError;
use helpers::helpers::{assert_err, contains_event};
use helpers::setup::TestEnv;
use osmosis_test_tube::{Module, Wasm};
use vaultenator::errors::ContractError;

#[test]
fn set_open() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let res = env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, false);
    assert!(contains_event(&res, "open_contract"))
}

#[test]
fn set_open_already_open() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    let res_err = env
        .set_open(&wasm, &contract_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::IsOpen {});
}

#[test]
fn set_open_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let res_err = env
        .set_open(&wasm, &contract_addr, &env.traders[0])
        .unwrap_err();

    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_pause() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, false);

    let res = env.set_pause(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, true);
    assert!(contains_event(&res, "paused"))
}

#[test]
fn set_pause_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, false);

    let res_err = env
        .set_pause(&wasm, &contract_addr, &env.traders[0])
        .unwrap_err();
    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_pause_contract_is_already_paused() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    env.set_pause(&wasm, &contract_addr, &env.signer).unwrap();
    let res_err = env
        .set_pause(&wasm, &contract_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::Paused {});
}

#[test]
fn set_unpause() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, false);

    env.set_pause(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, true);

    let res = env.set_unpause(&wasm, &contract_addr, &env.signer).unwrap();
    let state = env.query_state(&wasm, &contract_addr).unwrap();
    assert_eq!(state.is_open, true);
    assert_eq!(state.is_paused, false);
    assert!(contains_event(&res, "unpaused"))
}

#[test]
fn set_unpause_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();

    let res_err = env
        .set_unpause(&wasm, &contract_addr, &env.traders[0])
        .unwrap_err();
    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_unpause_contract_is_already_unpaused() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    env.set_open(&wasm, &contract_addr, &env.signer).unwrap();

    let res_err = env
        .set_unpause(&wasm, &contract_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::NotPaused {});
}
