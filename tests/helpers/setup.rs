use super::helpers::store_code;
use cosmwasm_std::{coin, Addr, Coin, Uint128};
use example_vault::config::MyConfig;
use example_vault::state::MyState;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use osmosis_test_tube::{
    osmosis_std::types::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryTotalSupplyRequest},
    Bank, Module, OsmosisTestApp, RunnerExecuteResult, RunnerResult, SigningAccount, Wasm,
};
use std::str::FromStr;
use vaultenator::msg::{
    ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, InstantiateMsg, QueryMsg,
    VaultenatorExtensionExecuteMsg, VaultenatorExtensionQueryMsg,
};
use vaultenator::ownership::OwnerProposal;

pub const PROPOSAL_DURATION: u64 = 1000;

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
        duration: u64,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let propose_new_owner_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::ProposeNewOwner {
                new_owner,
                duration,
            },
        ));

        wasm.execute(contract_addr, &propose_new_owner_msg, &[], signer)
    }

    pub fn deposit(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Coin,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Deposit {
            amount: Uint128::one(),
            recipient: None,
        };
        wasm.execute(contract_addr, &msg, &[amount], signer)
    }

    pub fn redeem(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Coin,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Redeem {
            amount: Uint128::one(),
            recipient: None,
        };
        wasm.execute(contract_addr, &msg, &[amount], signer)
    }

    pub fn claim_ownership(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let claim_ownership_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::ClaimOwnership {},
        ));
        wasm.execute(contract_addr, &claim_ownership_msg, &[], signer)
    }

    pub fn reject_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let reject_owner_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::RejectOwner {},
        ));
        wasm.execute(contract_addr, &reject_owner_msg, &[], signer)
    }

    pub fn query_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Addr> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::Owner {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_ownership_proposal(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<OwnerProposal> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::OwnershipProposal {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_config(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<MyConfig> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::Config {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_state(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<MyState> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::State {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn set_open(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_open_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::SetOpen {},
        ));
        wasm.execute(contract_addr, &set_open_msg, &[], signer)
    }

    pub fn set_pause(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_pause_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::Pause {},
        ));
        wasm.execute(contract_addr, &set_pause_msg, &[], signer)
    }

    pub fn set_unpause(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_unpause_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::UnPause {},
        ));
        wasm.execute(contract_addr, &set_unpause_msg, &[], signer)
    }

    pub fn get_balance(&self, address: &str, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address: address.to_string(),
                denom: denom.to_string(),
            })
            .unwrap();

        match response.balance {
            Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
            None => Uint128::zero(),
        }
    }

    pub fn get_total_supply(&self, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_total_supply(&QueryTotalSupplyRequest { pagination: None })
            .unwrap()
            .supply
            .into_iter()
            .find(|coin| coin.denom == denom.to_string())
            .unwrap();

        Uint128::from_str(&response.amount).unwrap_or(Uint128::zero())
    }
}
