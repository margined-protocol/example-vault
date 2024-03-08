use crate::errors::ContractError;
use crate::state::CONFIG;
use crate::structs::Config;
use cosmwasm_std::{DepsMut, Env, Reply, Response, SubMsgResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;

pub fn handle_create_strategy_denom_reply(
    deps: DepsMut,
    _env: Env,
    res: Reply,
) -> Result<Response, ContractError> {
    let sub_msg_response: SubMsgResult = res.result;
    let response: MsgCreateDenomResponse = sub_msg_response.try_into()?;

    CONFIG.update(
        deps.storage,
        |mut config| -> Result<Config, ContractError> {
            config.strategy_denom = Some(response.new_token_denom.clone());
            Ok(config)
        },
    )?;

    Ok(Response::new().add_attribute("strategy_denom", &response.new_token_denom))
}
