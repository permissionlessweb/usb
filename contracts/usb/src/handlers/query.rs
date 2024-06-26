use crate::{
    contract::{
        Usb, UsbResult
    },
    msg::{ConfigResponse, CountResponse, UsbQueryMsg},
    state::{CONFIG, COUNT},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _app: &Usb,
    msg: UsbQueryMsg,
) -> UsbResult<Binary> {
    match msg {
        UsbQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        UsbQueryMsg::Count {} => to_json_binary(&query_count(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let count = COUNT.load(deps.storage)?;
    Ok(CountResponse { count })
}
