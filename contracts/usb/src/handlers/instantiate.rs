use crate::{
    contract::{
        Usb, UsbResult
    },
    msg::UsbInstantiateMsg,
    state::{Config, CONFIG, COUNT},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: Usb,
    msg: UsbInstantiateMsg,
) -> UsbResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &msg.count)?;
    Ok(Response::new())
}
