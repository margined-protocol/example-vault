use super::helpers::store_code;
use cosmwasm_std::coin;
use osmosis_test_tube::{OsmosisTestApp, SigningAccount, Wasm};
use vaultenator::msg::InstantiateMsg;

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub signer: SigningAccount,
    pub traders: Vec<SigningAccount>,
}
impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();

        let signer = app
            .init_account(&[coin(1_000_000_000_000_000_000, "uosmo")])
            .unwrap();

        let mut traders: Vec<SigningAccount> = Vec::new();
        for _ in 0..10 {
            traders.push(
                app.init_account(&[coin(1_000_000_000_000_000_000, "uosmo")])
                    .unwrap(),
            );
        }

        Self {
            app,
            signer,
            traders,
        }
    }
    pub fn deploy_contract(&self, wasm: &Wasm<OsmosisTestApp>) -> String {
        let code_id = store_code(&wasm, &self.signer, env!("CARGO_PKG_NAME")).unwrap();
        wasm.instantiate(
            code_id,
            &InstantiateMsg {
                base_denom: "uosmo".to_string(),
            },
            None,
            Some("example-vault"),
            &[],
            &self.signer,
        )
        .unwrap()
        .data
        .address
    }
}
