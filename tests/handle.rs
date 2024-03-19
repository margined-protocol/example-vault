mod helpers;
extern crate example_vault;
use cosmwasm_std::{coin, Uint128};

use helpers::setup::TestEnv;
use osmosis_test_tube::{Account, Module, Wasm};

#[test]
fn deposit() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    let amount = coin(20_000_000u128, "uosmo".to_string());
    env.deposit(&wasm, &contract_addr, amount, &env.signer)
        .unwrap();
    let config = env.query_config(&wasm, &contract_addr).unwrap();
    let strategy_denom = config.strategy_denom.unwrap();

    let signer_strategy_denom_balance =
        env.get_balance(env.signer.address(), strategy_denom.clone());

    let contract_base_denom_balance = env.get_balance(contract_addr, config.base_denom);

    let base_denom_total_supply = env.get_total_supply(strategy_denom);

    assert_eq!(base_denom_total_supply, Uint128::from(20_000_000u128));
    assert_eq!(signer_strategy_denom_balance, Uint128::from(20_000_000u128));
    assert_eq!(contract_base_denom_balance, Uint128::from(20_000_000u128));
}
