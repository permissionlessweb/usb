use crate::{
    contract::{AdapterResult, UsbAdapter},
    msg::UsbAdapterExecuteMsg,
    state::{CONFIG, STATUS},
    UsbError, USB_NAMESPACE,
};

use abstract_adapter::{
    objects::namespace::Namespace,
    sdk::{AccountVerification, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use cosmwasm_std::{ensure_eq, DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    adapter: UsbAdapter,
    msg: UsbAdapterExecuteMsg,
) -> AdapterResult {
    match msg {
        UsbAdapterExecuteMsg::UpdateConfig {} => update_config(deps, info, adapter),
        UsbAdapterExecuteMsg::SetStatus { status } => set_status(deps, adapter, status),
    }
}

/// Update the configuration of the adapter
fn update_config(deps: DepsMut, _msg_info: MessageInfo, adapter: UsbAdapter) -> AdapterResult {
    // Only admin(namespace owner) can change recipient address
    let namespace = adapter
        .module_registry(deps.as_ref())?
        .query_namespace(Namespace::new(USB_NAMESPACE)?)?;

    // unwrap namespace, since it's unlikely to have unclaimed namespace as this adapter installed
    let namespace_info = namespace.unwrap();
    ensure_eq!(
        namespace_info.account_base,
        adapter.target_account.clone().unwrap(),
        UsbError::Unauthorized {}
    );
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(adapter.response("update_config"))
}

fn set_status(deps: DepsMut, adapter: UsbAdapter, status: String) -> AdapterResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &status)?;

    Ok(adapter
        .response("set_status")
        .add_attribute("new_status", &status)
        .add_attribute("account_id", account_id.to_string()))
}
