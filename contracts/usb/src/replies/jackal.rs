use abstract_app::sdk::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{Usb, UsbResult};

pub fn jackal_reply(_deps: DepsMut, _env: Env, app: Usb, _reply: Reply) -> UsbResult {
    Ok(app.response("instantiate_reply"))
}
