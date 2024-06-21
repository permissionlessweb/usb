use crate::{
    contract::{Usb, UsbResult},
    msg::{ UsbExecuteMsg},
    state::{CONFIG, COUNT},
    UsbError,
};

use abstract_app::{
    objects::{chain_name::ChainName, module::ModuleInfo},
    std::{ibc_client, IBC_CLIENT},
    traits::AbstractResponse,
};
use cosmwasm_std::{
    to_json_binary, wasm_execute, Addr, CosmosMsg, DepsMut, Empty, Env, MessageInfo,
};
use prost::Message;
use usb::{
    JackalMsg,
    helpers::{hash_and_hex, merkle_helper},
    types::{filetree::MsgPostFile, storage::MsgSignContract},
};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: Usb,
    msg: UsbExecuteMsg,
) -> UsbResult {
    match msg {
        UsbExecuteMsg::UpdateConfig {} => update_config(deps, info, app),
        UsbExecuteMsg::Increment {} => increment(deps, app),
        UsbExecuteMsg::Reset { count } => reset(deps, info, count, app),
        UsbExecuteMsg::SendContent { msg } => send_content(info, msg, app),
    }
}
fn send_content(info: MessageInfo, msg: JackalMsg, mut app: Usb) -> UsbResult {
    // define msgs to send to jackal as account
    let msg = match msg {
        JackalMsg::MakeRoot {
            editors,
            viewers,
            tracking_number,
        } => {
            let make_root_type = String::from("/canine_chain.filetree.MsgMakeRootV2");
            let msg_make_root = usb::types::filetree::MsgMakeRootV2 {
                creator: info.sender.to_string(), //todo: change to account manager or proxy
                viewers,
                editors,
                tracking_number,
            };
            let encoded_root_msg = msg_make_root.encode_to_vec();
            let make_root_msg: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: make_root_type.clone(),
                value: cosmwasm_std::Binary(encoded_root_msg.to_vec()),
            };

            Ok(make_root_msg)
        }
        JackalMsg::PostFile {
            hash_parent,
            hash_child,
            contents,
            viewers,
            editors,
            tracking_number,
        } => {
            let post_file_type = String::from("/canine_chain.filetree.MsgPostFile");
            let msg_post_file = MsgPostFile {
                creator: info.sender.to_string(),
                account: hash_and_hex(&info.sender.to_string()), // todo: derive from manager
                hash_parent,
                hash_child,
                contents,
                viewers,
                editors,
                tracking_number,
            };
            let encoded_file = msg_post_file.encode_to_vec();
            let file: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: post_file_type.clone(),
                value: cosmwasm_std::Binary(encoded_file.to_vec()),
            };
            Ok(file)
        }
        JackalMsg::SignContract { cid } => {
            let sign_contract_type = String::from("/canine_chain.storage.MsgSignContract");
            let msg_sign_contract = MsgSignContract {
                creator: info.sender.to_string(),
                cid,
            };
            let encoded_sign_contract = msg_sign_contract.encode_to_vec();
            let sign_contract: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: sign_contract_type.clone(),
                value: cosmwasm_std::Binary(encoded_sign_contract.to_vec()),
            };
            Ok(sign_contract)
        }

        _ => Err(UsbError::NotImplemented()),
    }
    .map_err(|error| error)?;

    Ok(app.response("send_content").add_message(msg))
}

// pub(crate) fn route_msg(app: Usb, sender: Addr, msg: JackalMsg) -> UsbResult<CosmosMsg> {

//     // let current_module_info = ModuleInfo::from_id(app.module_id(), app.version().into())?;
//     // // Call IBC client
//     // let ibc_client_msg = ibc_client::ExecuteMsg::ModuleIbcAction {
//     //     host_chain: ChainName::from_string("jackal".to_string())?.to_string(),
//     //     target_module: current_module_info,
//     //     msg: to_json_binary(&ServerIbcMessage::RouteMessage { msg, header })?,
//     //     callback_info: None,
//     // };

//     // let ibc_client_addr: Addr = app
//     //     .module_registry(deps.as_ref())?
//     //     .query_module(ModuleInfo::from_id_latest(IBC_CLIENT)?)?
//     //     .reference
//     //     .unwrap_native()?;

//     // let msg: CosmosMsg = wasm_execute(ibc_client_addr, &ibc_client_msg, vec![])?.into();
// }

/// Update the configuration of the app
fn update_config(deps: DepsMut, msg_info: MessageInfo, app: Usb) -> UsbResult {
    // Only the admin should be able to call this
    app.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(app.response("update_config"))
}

fn increment(deps: DepsMut, app: Usb) -> UsbResult {
    COUNT.update(deps.storage, |count| UsbResult::Ok(count + 1))?;

    Ok(app.response("increment"))
}

fn reset(deps: DepsMut, info: MessageInfo, count: i32, app: Usb) -> UsbResult {
    app.admin.assert_admin(deps.as_ref(), &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(app.response("reset"))
}
