use super::helpers::store_code;
use cosmwasm_std::coin;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use osmosis_test_tube::{OsmosisTestApp, RunnerExecuteResult, SigningAccount, Wasm};
use vaultenator::msg::{
    ExecuteMsg, ExtensionExecuteMsg, InstantiateMsg, MarginedExtensionExecuteMsg,
};
const PROPOSAL_DURATION: u64 = 1000;

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

    pub fn propose_new_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        new_owner: String,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let propose_new_owner_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Margined(
            MarginedExtensionExecuteMsg::ProposeNewOwner {
                new_owner,
                duration: PROPOSAL_DURATION,
            },
        ));

        wasm.execute(contract_addr, &propose_new_owner_msg, &[], signer)
    }

    pub fn claim_ownership(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let claim_ownership_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Margined(
            MarginedExtensionExecuteMsg::ClaimOwnership {},
        ));
        wasm.execute(contract_addr, &claim_ownership_msg, &[], signer)
    }
}
