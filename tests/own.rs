mod helpers;
extern crate example_vault;
use helpers::helpers::assert_err;
use helpers::setup::TestEnv;
use osmosis_test_tube::{Account, Module, Wasm};
use vaultenator::errors::ContractError;

#[test]
fn claim_ownership_no_proposal() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let res_err = env
        .claim_ownership(&wasm, &contract_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::ProposalNotFound {});
}

#[test]
fn propose_new_owner_claim_unauthorised() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    env.propose_new_owner(&wasm, &contract_addr, env.traders[0].address(), &env.signer)
        .unwrap();

    let res_err = env
        .claim_ownership(&wasm, &contract_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::Unauthorized {});
}
