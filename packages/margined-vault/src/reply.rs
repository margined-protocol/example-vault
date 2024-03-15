use crate::config::Configure;
use crate::errors::ContractError;
use cosmwasm_std::{DepsMut, Env, Reply, Response, SubMsgResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub const CREATE_STRATEGY_DENOM_REPLY_ID: u64 = 1u64;

pub trait ReplyHandler<C>
where
    C: Configure + Serialize + DeserializeOwned,
{
    fn handle_reply(
        &self,
        mut deps: DepsMut,
        _env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        match msg.id {
            CREATE_STRATEGY_DENOM_REPLY_ID => {
                let sub_msg_response: SubMsgResult = msg.result;
                let response: MsgCreateDenomResponse = sub_msg_response.try_into()?;

                let mut config = C::get_from_storage(deps.as_ref())?;
                config.update_strategy_denom(response.new_token_denom.clone());
                config.save_to_storage(&mut deps)?;

                Ok(Response::new().add_attribute("strategy_denom", &response.new_token_denom))
            }
            _ => Err(ContractError::InvalidReplyId),
        }
    }
}
