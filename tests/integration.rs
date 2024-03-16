mod helpers;
extern crate example_vault;
use cw_vault_standard::{VaultStandardInfoResponse, VaultStandardQueryMsg};
use helpers::setup::TestEnv;
use osmosis_test_tube::{Module, Wasm};
use vaultenator::msg::QueryMsg;

#[test]
fn query_standard_info() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let contract_address = env.deploy_contract(&wasm);

    let vault_info = wasm
        .query::<QueryMsg, VaultStandardInfoResponse>(
            &contract_address,
            &VaultStandardQueryMsg::VaultStandardInfo {},
        )
        .unwrap();
    assert_eq!(vault_info.version, 1);
    assert_eq!(vault_info.extensions.len(), 2);
    let expected_extensions = vec!["lockup".to_string(), "force-unlock".to_string()];
    assert_eq!(vault_info.extensions, expected_extensions);
}
