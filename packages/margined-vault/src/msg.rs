use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg};
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_denom: String,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    Margined(MarginedExtensionExecuteMsg),
}

#[cw_serde]
pub enum ExtensionQueryMsg {
    Margined(MarginedExtensionQueryMsg),
}

#[cw_serde]
pub enum MarginedExtensionQueryMsg {
    Config {},
    State {},
}

#[cw_serde]
pub enum MarginedExtensionExecuteMsg {
    ClaimOwnership {},
    Pause {},
    ProposeNewOwner { new_owner: String, duration: u64 },
    RejectOwner {},
    SetOpen {},
    UpdateConfig {},
    UnPause {},
}

pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

pub fn create_denom_message(contract_address: &Addr, subdenom: String) -> CosmosMsg {
    MsgCreateDenom {
        sender: contract_address.to_string(),
        subdenom,
    }
    .into()
}
