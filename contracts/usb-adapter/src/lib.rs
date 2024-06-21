pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::UsbInterface;
pub use error::UsbError;
pub use msg::{
    UsbAdapterExecuteMsg, UsbAdapterInstantiateMsg
};

/// The version of your Adapter
pub const ADAPTER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const USB_NAMESPACE: &str = "usb";
pub const USB_NAME: &str = "usb";
pub const USB_ID: &str = const_format::concatcp!(USB_NAMESPACE, ":", USB_NAME);
