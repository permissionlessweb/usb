use crate::{
    contract::{Usb, UsbResult},
    msg::UsbExecuteMsg,
    state::{CONFIG, COUNT},
    UsbError,
};

use abstract_app::{
    objects::{chain_name::ChainName, module::ModuleInfo},
    sdk::{Execution, IbcInterface},
    std::{ibc_client, ibc_host::HostAction, manager, proxy, IBC_CLIENT, PROXY},
    traits::AbstractResponse,
};
use cosmwasm_std::{
    to_json_binary, wasm_execute, Addr, CosmosMsg, DepsMut, Empty, Env, MessageInfo,
};
use prost::Message;
use usb::{
    helpers::{hash_and_hex, merkle_helper},
    types::{
        filetree::{MsgAddViewers, MsgDeleteViewers, MsgPostFile, MsgPostKey},
        storage::{MsgBuyStorage, MsgCancelContract, MsgSignContract, MsgUpgradeStorage},
    },
    JackalMsg,
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
        UsbExecuteMsg::SendContent { msg } => send_content(deps, info, msg, app),
    }
}
// content workflow: manager -> usb -> proxy -> ibc-client -> note -> (ibc) -> voice -> proxy -> ibc-host -> jackal
fn send_content(deps: DepsMut, info: MessageInfo, msg: JackalMsg, mut app: Usb) -> UsbResult {
    // api for executing account actions
    let executor = app.executor(deps.as_ref());
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
        JackalMsg::AddViewers {
            viewer_ids,
            viewer_keys,
            address,
            owner,
        } => {
            let add_viewers_type = String::from("/canine_chain.filetree.MsgAddViewers");
            let msg_add_viewers = MsgAddViewers {
                creator: info.sender.to_string(),
                viewer_ids,
                viewer_keys,
                address,
                owner,
            };
            let encoded_add_viewers = msg_add_viewers.encode_to_vec();
            let add_viewers = CosmosMsg::Stargate {
                type_url: add_viewers_type,
                value: cosmwasm_std::Binary(encoded_add_viewers),
            };
            Ok(add_viewers)
        }
        JackalMsg::DeleteViewers {
            viewer_ids,
            address,
            owner,
        } => {
            let delete_viewers_type = String::from("/canine_chain.filetree.MsgRemoveViewers");
            let msg_delete_viewers = MsgDeleteViewers {
                creator: info.sender.to_string(),
                viewer_ids,
                address,
                owner,
            };
            let encoded_add_viewers = msg_delete_viewers.encode_to_vec();
            let delete_viewers = CosmosMsg::Stargate {
                type_url: delete_viewers_type,
                value: cosmwasm_std::Binary(encoded_add_viewers),
            };
            Ok(delete_viewers)
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
        JackalMsg::CancelContract { cid } => {
            let cancel_contract_type = String::from("/canine_chain.storage.MsgCancelContract");
            let msg_cancel_contract = MsgCancelContract {
                creator: info.sender.to_string(),
                cid,
            };

            let encoded_cancel_contract = msg_cancel_contract.encode_to_vec();
            let cancel_contract: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: cancel_contract_type,
                value: cosmwasm_std::Binary(encoded_cancel_contract),
            };
            Ok(cancel_contract)
        }
        JackalMsg::BuyStorage {
            for_address,
            duration_days,
            bytes,
            payment_denom,
        } => {
            let buy_storage_type = String::from("/canine_chain.storage.MsgBuyStorage");
            let msg_buy_storage = MsgBuyStorage {
                creator: info.sender.to_string(),
                for_address,
                duration_days,
                bytes,
                payment_denom,
            };
            let encoded_buy_storage = msg_buy_storage.encode_to_vec();

            let buy_storage: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: buy_storage_type,
                value: cosmwasm_std::Binary(encoded_buy_storage),
            };
            Ok(buy_storage)
        }
        JackalMsg::UpgradeStorage {
            for_address,
            duration_days,
            bytes,
            payment_denom,
        } => {
            let upgrade_storage_type = String::from("/canine_chain.storage.MsgUpgradeStorage");
            let msg_upgrade_storage = MsgUpgradeStorage {
                creator: info.sender.to_string(),
                for_address,
                duration_days,
                bytes,
                payment_denom,
            };
            let encoded_upgrade_storage = msg_upgrade_storage.encode_to_vec();
            let upgrade_storage: CosmosMsg<Empty> = CosmosMsg::Stargate {
                type_url: upgrade_storage_type,
                value: cosmwasm_std::Binary(encoded_upgrade_storage),
            };
            Ok(upgrade_storage)
        }
        JackalMsg::PostKey { key } => {
            let post_key_type = String::from("/canine_chain.storage.MsgPostKey");
            let msg_post_key = MsgPostKey {
                key,
                creator: info.sender.to_string(),
            };

            let encoded_post_key = msg_post_key.encode_to_vec();
            let post_key = CosmosMsg::Stargate {
                type_url: post_key_type,
                value: cosmwasm_std::Binary(encoded_post_key),
            };
            Ok(post_key)
        }
        _ => Err(UsbError::NotImplemented()),
    }
    .map_err(|error| error)?;

    // sends msg to ibc-client for ibc transfer & execution on jackal
    let send_as_proxy: CosmosMsg = wasm_execute(
        app.ibc_client(deps.as_ref()).module_address()?,
        &proxy::ExecuteMsg::IbcAction {
            msg: ibc_client::ExecuteMsg::RemoteAction {
                host_chain: ChainName::from_string("juno".to_string())?.to_string(),
                action: HostAction::Dispatch {
                    manager_msgs: vec![manager::ExecuteMsg::ExecOnModule {
                        module_id: PROXY.to_string(),
                        exec_msg: to_json_binary(&proxy::ExecuteMsg::ModuleAction {
                            msgs: vec![msg],
                        })?,
                    }],
                },
            },
        },
        info.funds,
    )?
    .into();

    Ok(app.response("send_content").add_message(send_as_proxy))
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
