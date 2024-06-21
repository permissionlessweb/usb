use crate::contract::{
    Usb, UsbResult
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(_deps: DepsMut, _env: Env, app: Usb, _reply: Reply) -> UsbResult {
    Ok(app.response("instantiate_reply"))
}
