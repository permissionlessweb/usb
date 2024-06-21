use crate::{
    msg::{
        UsbAdapterExecuteMsg, UsbAdapterQueryMsg
    },
    USB_ID,
};

use abstract_adapter::sdk::{
    features::{AccountIdentification, Dependencies, ModuleIdentification},
    AbstractSdkResult, AdapterInterface,
};
use abstract_adapter::std::objects::module::ModuleId;
use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{CosmosMsg, Deps, Uint128};

// API for Abstract SDK users
/// Interact with your adapter in other modules.
pub trait UsbApi: AccountIdentification + Dependencies + ModuleIdentification {
    /// Construct a new adapter interface.
    fn usb<'a>(&'a self, deps: Deps<'a>) -> UsbAdapter<Self> {
        UsbAdapter {
            base: self,
            deps,
            module_id: USB_ID,
        }
    }
}

impl<T: AccountIdentification + Dependencies + ModuleIdentification> UsbApi for T {}

#[derive(Clone)]
pub struct UsbAdapter<'a, T: UsbApi> {
    pub base: &'a T,
    pub module_id: ModuleId<'a>,
    pub deps: Deps<'a>,
}

impl<'a, T: UsbApi> UsbAdapter<'a, T> {
    /// Set the module id
    pub fn with_module_id(self, module_id: ModuleId<'a>) -> Self {
        Self { module_id, ..self }
    }

    /// returns the HUB module id
    fn module_id(&self) -> ModuleId {
        self.module_id
    }

    /// Executes a [UsbExecuteMsg] in the adapter
    fn request(&self, msg: UsbAdapterExecuteMsg) -> AbstractSdkResult<CosmosMsg> {
        let adapters = self.base.adapters(self.deps);

        adapters.execute(self.module_id(), msg)
    }

    /// Route message
    pub fn update_config(&self) -> AbstractSdkResult<CosmosMsg> {
        self.request(UsbAdapterExecuteMsg::UpdateConfig {})
    }
}

/// Queries
impl<'a, T: UsbApi> UsbAdapter<'a, T> {
    /// Query your adapter via message type
    pub fn query<R: DeserializeOwned>(&self, query_msg: UsbAdapterQueryMsg) -> AbstractSdkResult<R> {
        let adapters = self.base.adapters(self.deps);
        adapters.query(self.module_id(), query_msg)
    }

    /// Query config
    pub fn config(&self) -> AbstractSdkResult<Uint128> {
        self.query(UsbAdapterQueryMsg::Config {})
    }
}
