extern crate margined_vault;
use super::helpers::store_code;
use cosmwasm_std::Coin;
use margined_vault::msg::InstantiateMsg;
use osmosis_test_tube::{Module, OsmosisTestApp, SigningAccount, Wasm};

pub struct ContractInfo {
    pub addr: String,
    pub id: u64,
}

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub signer: SigningAccount,
}
impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(1_000_000_000_000, "uosmo")])
            .unwrap();

        Self { app, signer }
    }
}

pub trait ContractTester {
    fn initialize(&mut self) -> Result<ContractInfo, String>;
}

impl ContractTester for TestEnv {
    fn initialize(&mut self) -> Result<ContractInfo, String> {
        let wasm = Wasm::new(&self.app);
        let code_id = store_code(&wasm, &self.signer, env!("CARGO_PKG_NAME")).unwrap();

        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    base_denom: "uosmo".to_string(),
                },
                None,
                Some("test-macro-vault"),
                &[],
                &self.signer,
            )
            .unwrap()
            .data
            .address;

        Ok(ContractInfo {
            addr: contract_addr,
            id: code_id,
        })
    }
}
