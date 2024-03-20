use crate::config::MyConfig;
use crate::contract::MyVault;
use crate::state::MyState;
use cosmwasm_std::{coin, BankMsg, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response};
use cw_utils::must_pay;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use vaultenator::config::Configure;
use vaultenator::errors::ContractError;
use vaultenator::handlers::Handle;
use vaultenator::state::{ManageState, OWNER};

impl Handle<MyConfig, MyState> for MyVault {
    fn handle_update_config(
        &self,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        // let config = MyConfig::get_from_storage(deps.as_ref())?;
        // config.strategy_cap = new_config.strategy_cap;

        // config.save_to_storage(&mut deps)?;

        Ok(Response::new().add_event(Event::new("update_config")))
    }
    fn handle_deposit(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        MyState::is_open_and_unpaused(deps.as_ref())?;
        let config = MyConfig::get_from_storage(deps.as_ref())?;

        let amount =
            must_pay(&info, &config.base_denom).map_err(|_| ContractError::InvalidFunds {})?;

        let strategy_denom = config
            .strategy_denom
            .as_ref()
            .ok_or(ContractError::DenomNotInitialized {})?;

        let msg = MsgMint {
            sender: env.contract.address.to_string(),
            amount: Some(OsmosisCoin {
                denom: strategy_denom.to_string(),
                amount: amount.to_string(),
            }),
            mint_to_address: info.sender.to_string(),
        };

        Ok(Response::default().add_message(msg))
    }
    fn handle_redeem(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        MyState::is_open_and_unpaused(deps.as_ref())?;
        let config = MyConfig::get_from_storage(deps.as_ref())?;

        let strategy_denom = config
            .strategy_denom
            .ok_or(ContractError::DenomNotInitialized {})?;

        let strategy_denom_amount =
            must_pay(&info, &strategy_denom).map_err(|_| ContractError::InvalidFunds {})?;

        let burn_strategy_token_msg = MsgBurn {
            sender: env.contract.address.to_string(),
            amount: Some(OsmosisCoin {
                denom: strategy_denom.to_string(),
                amount: strategy_denom_amount.to_string(),
            }),
            burn_from_address: env.contract.address.to_string(),
        };

        let msg_transfer = CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(strategy_denom_amount.into(), config.base_denom)],
        });

        let mut response = Response::default();

        response = response
            .add_message(burn_strategy_token_msg)
            .add_message(msg_transfer);

        Ok(response)
    }
}
