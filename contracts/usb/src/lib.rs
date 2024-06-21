pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;

pub use error::UsbError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::UsbInterface;

pub const USB_NAMESPACE: &str = "usb";
pub const JBUSB_NAME: &str = "usb";
pub const USB_ID: &str = const_format::concatcp!(USB_NAMESPACE, ":", JBUSB_NAME);
