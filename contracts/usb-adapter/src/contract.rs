use crate::{
    error::UsbError,
    handlers,
    msg::{
        UsbAdapterExecuteMsg, UsbAdapterInstantiateMsg, UsbAdapterQueryMsg
    },
    ADAPTER_VERSION, USB_ID,
};

use abstract_adapter::AdapterContract;
use cosmwasm_std::Response;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type UsbAdapter = AdapterContract<
    UsbError,
    UsbAdapterInstantiateMsg,
    UsbAdapterExecuteMsg,
    UsbAdapterQueryMsg,
>;
/// The type of the result returned by your Adapter's entry points.
pub type AdapterResult<T = Response> = Result<T, UsbError>;

const USB_ADAPTER: UsbAdapter = UsbAdapter::new(USB_ID, ADAPTER_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler);

// Export handlers
#[cfg(feature = "export")]
abstract_adapter::export_endpoints!(USB_ADAPTER, UsbAdapter);

abstract_adapter::cw_orch_interface!(
    USB_ADAPTER,
    UsbAdapter,
    UsbAdapterInstantiateMsg,
    UsbInterface
);
