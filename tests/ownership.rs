mod helpers;
extern crate example_vault;
use helpers::helpers::assert_err;
use helpers::setup::{TestEnv, PROPOSAL_DURATION};
use osmosis_test_tube::{Account, Module, Wasm};
use vaultenator::errors::ContractError;
use vaultenator::ownership::MAX_DURATION;

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

    env.propose_new_owner(
        &wasm,
        &contract_addr,
        env.traders[0].address(),
        PROPOSAL_DURATION,
        &env.signer,
    )
    .unwrap();

    let res_err = env
        .claim_ownership(&wasm, &contract_addr, &env.traders[1])
        .unwrap_err();

    assert_err(res_err, ContractError::Unauthorized {});
}

#[test]
fn propose_new_owner_expiry_too_long() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);

    let res_err = env
        .propose_new_owner(
            &wasm,
            &contract_addr,
            env.traders[0].address(),
            MAX_DURATION + 1,
            &env.signer,
        )
        .unwrap_err();

    assert_err(res_err, ContractError::InvalidDuration(MAX_DURATION));
}

#[test]
fn query_ownership_proposal() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    env.propose_new_owner(
        &wasm,
        &contract_addr,
        env.traders[0].address(),
        PROPOSAL_DURATION,
        &env.signer,
    )
    .unwrap();

    let ownership_proposal = env.query_ownership_proposal(&wasm, &contract_addr).unwrap();

    let timestamp = env.app.get_block_time_seconds();

    assert_eq!(ownership_proposal.owner, env.traders[0].address());
    assert_eq!(
        ownership_proposal.expiry,
        timestamp as u64 + PROPOSAL_DURATION
    );
}

#[test]
fn propose_new_owner_claim_success() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    let owner = env.query_owner(&wasm, &contract_addr).unwrap();

    assert_eq!(owner.to_string(), env.signer.address());

    env.propose_new_owner(
        &wasm,
        &contract_addr,
        env.traders[0].address(),
        PROPOSAL_DURATION,
        &env.signer,
    )
    .unwrap();

    env.claim_ownership(&wasm, &contract_addr, &env.traders[0])
        .unwrap();

    let new_owner = env.query_owner(&wasm, &contract_addr).unwrap();

    assert_eq!(new_owner.to_string(), env.traders[0].address());
}

#[test]
fn propose_new_owner_expired_proposal() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_addr = env.deploy_contract(&wasm);
    let owner = env.query_owner(&wasm, &contract_addr).unwrap();

    assert_eq!(owner.to_string(), env.signer.address());

    env.propose_new_owner(
        &wasm,
        &contract_addr,
        env.traders[0].address(),
        PROPOSAL_DURATION,
        &env.signer,
    )
    .unwrap();

    env.app.increase_time(PROPOSAL_DURATION + 1);

    let res_err = env
        .claim_ownership(&wasm, &contract_addr, &env.traders[0])
        .unwrap_err();

    assert_err(res_err, ContractError::Expired {});
}
