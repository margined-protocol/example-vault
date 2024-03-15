use crate::errors::ContractError;
// use crate::maths::{calc_base_to_withdraw, calc_strategy_share};
// use crate::queries::get_total_supply;
use crate::config::Configure;
use crate::contract::Describe;
use crate::msg::{create_denom_message, InstantiateMsg};
use crate::state::ManageState;
use crate::state::{CONTRACT_VERSION, OWNER};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, SubMsg};
use cw2::set_contract_version;

// use osmosis_std::types::{
//     cosmos::base::v1beta1::Coin as OsmosisCoin,
//     osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
// };
pub const CREATE_STRATEGY_DENOM_REPLY_ID: u64 = 1u64;

// pub fn mint_strategy_token(
//     deps: &DepsMut,
//     env: &Env,
//     to: String,
//     amount: Uint128,
// ) -> Result<MsgMint, ContractError> {
//     let config = CONFIG.load(deps.storage)?;
//     let strategy_denom = config
//         .strategy_denom
//         .as_ref()
//         .ok_or(ContractError::DenomNotInitialized {})?;
//     let msg = MsgMint {
//         sender: env.contract.address.to_string(),
//         amount: Some(OsmosisCoin {
//             denom: strategy_denom.to_string(),
//             amount: amount.to_string(),
//         }),
//         mint_to_address: to,
//     };
//     Ok(msg)
// }

// pub fn burn_strategy_token(
//     deps: &DepsMut,
//     env: &Env,
//     amount: Uint128,
// ) -> Result<MsgBurn, ContractError> {
//     let config = CONFIG.load(deps.storage)?;
//     let strategy_denom = config
//         .strategy_denom
//         .as_ref()
//         .ok_or(ContractError::DenomNotInitialized {})?;
//     let msg = MsgBurn {
//         sender: env.contract.address.to_string(),
//         amount: Some(OsmosisCoin {
//             denom: strategy_denom.to_string(),
//             amount: amount.to_string(),
//         }),
//         burn_from_address: env.contract.address.to_string(),
//     };
//     Ok(msg)
// }

// fn check_strategy_cap(
//     deps: &DepsMut,
//     deposit_amount: Uint128,
//     strategy_collateral: Uint128,
// ) -> Result<bool, ContractError> {
//     let config: Config = CONFIG.load(deps.storage)?;

//     if strategy_collateral + deposit_amount > config.strategy_cap {
//         return Err(ContractError::StrategyCapExceeded {});
//     }
//     Ok(true)
// }

// pub fn withdraw(
//     deps: DepsMut,
//     _env: &Env,
//     withdrawer: Addr,
//     strategy_denom_amount: Uint128,
// ) -> Result<bool, ContractError> {
//     let config: Config = CONFIG.load(deps.storage)?;

//     // Get the total supply of the strategy token
//     let strategy_denom = config
//         .strategy_denom
//         .as_ref()
//         .ok_or(ContractError::DenomNotInitialized {})?;
//     let strategy_denom_total_supply = get_total_supply(&deps, strategy_denom)?;

//     // Calculate the strategy share for the provided amount of strategy tokens
//     let strategy_share = calc_strategy_share(strategy_denom_amount, strategy_denom_total_supply)?;

//     // Calc total_base in some way

//     // Compute the amount of collateral to redeem
//     // let base_to_withdraw = calc_base_to_withdraw(strategy_share, total_base)?;

//     Ok(true)
// }
// pub trait Handle: Describe {
//     fn handle_instantiate(
//         mut deps: DepsMut,
//         env: Env,
//         info: MessageInfo,
//         msg: InstantiateMsg,
//     ) -> Result<Response, ContractError> {
//         crate::handlers::handle_instantiate(deps, env, info, msg)
//     }
// }

pub trait Handle<C, S>: Describe
where
    C: Configure,
    S: ManageState,
{
    fn handle_instantiate(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(
            deps.storage,
            format!("crates.io:{}", Self::CONTRACT_NAME),
            CONTRACT_VERSION,
        )?;

        C::init_config(&mut deps, &msg)?;
        S::init_state(&mut deps, &env)?;

        OWNER.set(deps, Some(info.sender))?;

        let create_denom_sub_msg = SubMsg::reply_always(
            create_denom_message(&env.contract.address, Self::CONTRACT_NAME.to_string()),
            CREATE_STRATEGY_DENOM_REPLY_ID,
        );

        Ok(Response::new()
            .add_submessages([create_denom_sub_msg])
            .add_attribute("action", "instantiate"))
    }

    fn handle_update_config(
        &self,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;

    fn handle_deposit(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;
    fn handle_redeem(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;
}
