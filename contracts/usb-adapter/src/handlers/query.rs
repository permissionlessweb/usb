use crate::{
    contract::{AdapterResult, UsbAdapter},
    msg::{ConfigResponse,UsbAdapterQueryMsg, StatusResponse},
    state::{CONFIG, STATUS},
};

use abstract_adapter::objects::AccountId;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _adapter: &UsbAdapter,
    msg: UsbAdapterQueryMsg,
) -> AdapterResult<Binary> {
    match msg {
        UsbAdapterQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        UsbAdapterQueryMsg::Status { account_id } => {
            to_json_binary(&query_status(deps, account_id)?)
        }
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_status(deps: Deps, account_id: AccountId) -> StdResult<StatusResponse> {
    let status = STATUS.may_load(deps.storage, &account_id)?;
    Ok(StatusResponse { status })
}
