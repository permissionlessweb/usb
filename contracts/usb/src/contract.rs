use crate::{
    error::UsbError,
    handlers,
    msg::{
        UsbExecuteMsg, UsbInstantiateMsg, UsbMigrateMsg, UsbQueryMsg
    },
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, USB_ID,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

/// The type of the result returned by your app's entry points.
pub type UsbResult<T = Response> = Result<T, UsbError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type Usb =
    AppContract<UsbError, UsbInstantiateMsg, UsbExecuteMsg, UsbQueryMsg, UsbMigrateMsg>;

const APP: Usb = Usb::new(USB_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_dependencies(&[])
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, Usb);

abstract_app::cw_orch_interface!(APP, Usb, UsbInterface);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::UsbInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
