use crate::contract::Usb;

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;
use usb::JackalMsg;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(Usb, UsbExecuteMsg, UsbQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct UsbInstantiateMsg {}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum UsbExecuteMsg {
    JackalMsgs { msgs: Vec<JackalMsg> },
}

#[cosmwasm_schema::cw_serde]
pub struct UsbMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum UsbQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CountResponse)]
    Count {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct CountResponse {
    pub count: i32,
}
