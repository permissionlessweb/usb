use abstract_cw_orch_polytone::Polytone;
use abstract_interface::{Abstract, AccountFactoryExecFns};
use cw_orch::{anyhow::Result, contract::Deploy, mock::MockBase, prelude::*};
use cw_orch_interchain::{
    IbcQueryHandler, InterchainEnv, InterchainError, MockBech32InterchainEnv, MockInterchainEnv,
};
use polytone::handshake::POLYTONE_VERSION;
use usb_plugin::{
    contract::interface::UsbInterface,
    msg::{ConfigResponse, CountResponse, UsbExecuteMsgFns, UsbInstantiateMsg, UsbQueryMsgFns},
    UsbError, USB_NAMESPACE,
};

use abstract_app::{
    objects::{chain_name::ChainName, namespace::Namespace},
    std::{
        ibc_client::{ExecuteMsgFns, QueryMsgFns},
        ibc_host::ExecuteMsgFns as IbcHostExecuteMsgFunctions,
    },
};
use abstract_client::{AbstractClient, Application, Environment};
use cosmwasm_std::coins;
// Use prelude to get all the necessary imports
use abstract_std::ibc_client::{
    ExecuteMsgFns as IbcClientExecuteMsgFns, QueryMsgFns as IBCClientQueryFns,
};

struct TestEnv<Env: CwEnv> {
    env: Env,
    abs: AbstractClient<Env>,
    client1: Application<Env, UsbInterface<Env>>,
    client2: Application<Env, UsbInterface<Env>>,
}

impl<Env: CwEnv> TestEnv<Env> {
    /// Set up the test environment with an Account that has the App installed
    fn setup(env: Env) -> Result<TestEnv<Env>> {
        let namespace = Namespace::new(USB_NAMESPACE)?;

        // Abstract builder
        let abs_client = AbstractClient::builder(env.clone()).build()?;
        let publisher = abs_client.publisher_builder(namespace).build()?;
        publisher.publish_app::<UsbInterface<_>>()?;

        // Build Account
        let acc = abs_client
            .account_builder()
            .install_on_sub_account(false)
            .build()?;
        // Install USB
        let app = acc.install_app_with_dependencies::<UsbInterface<_>>(
            &UsbInstantiateMsg {},
            Empty {},
            &[],
        )?;
        // Build Account
        let acc2 = abs_client
            .account_builder()
            .install_on_sub_account(false)
            .build()?;
        // Install USB
        let app2 = acc2.install_app_with_dependencies::<UsbInterface<_>>(
            &UsbInstantiateMsg {},
            Empty {},
            &[],
        )?;

        // app.authorize_on_adapters(adapter_ids);

        Ok(TestEnv {
            env,
            abs: abs_client,
            client1: app,
            client2: app2,
        })
    }

    fn enable_ibc(&self) -> Result<()> {
        Polytone::deploy_on(self.abs.environment().clone(), None)?;
        Ok(())
    }
}

mod basic_functions {
    use usb::{
        types::filetree::{MsgMakeRootV2, MsgPostKey},
        JackalMsg,
    };

    use super::*;

    #[test]
    fn save_file() -> Result<()> {
        // Create a sender and mock env
        let interchain = MockBech32InterchainEnv::new(vec![
            ("juno-1", "bitsong1tzxe4deaztafjggza09mksn29hsd5nl32e945f"),
            ("jackal-1", "jkl1tyl97ac3s7sec4jwznk0s7n3tlwf3math03qj4"),
        ]);
        let bs_env = TestEnv::setup(interchain.chain("juno-1")?)?;
        let jkl_env = TestEnv::setup(interchain.chain("jackal-1")?)?;

        bs_env.enable_ibc()?;
        jkl_env.enable_ibc()?;

        ibc_connect_polytone_and_abstract(&interchain, "juno-1", "jackal-1")?;

        let bs_client = bs_env.client1;
        let jkl_client = jkl_env.client1;

        let msg = JackalMsg::MakeRoot {
            editors: "test".to_string(),
            viewers: "test".to_string(),
            tracking_number: "test".to_string(),
        };

        // Jackal storage workflow
        // 1. ?
        // 2. ?
        // 3. ?
        // 4. ?
        let res = bs_client.jackal_msgs(vec![msg])?;
        println!("{:#?}", res);

        Ok(())
    }
}

pub fn ibc_connect_polytone_and_abstract<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    interchain: &IBC,
    origin_chain_id: &str,
    remote_chain_id: &str,
) -> Result<()> {
    let origin_chain = interchain.chain(origin_chain_id).unwrap();
    let remote_chain = interchain.chain(remote_chain_id).unwrap();

    let abstr_origin = Abstract::load_from(origin_chain.clone())?;
    let abstr_remote = Abstract::load_from(remote_chain.clone())?;

    let origin_polytone = Polytone::load_from(origin_chain.clone())?;
    let remote_polytone = Polytone::load_from(remote_chain.clone())?;

    // Creating a connection between 2 polytone deployments
    interchain.create_contract_channel(
        &origin_polytone.note,
        &remote_polytone.voice,
        POLYTONE_VERSION,
        None, // Unordered channel
    )?;
    // Create the connection between client and host
    abstract_ibc_connection_with(&abstr_origin, interchain, &abstr_remote, &origin_polytone)?;
    Ok(())
}

pub fn abstract_ibc_connection_with<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    abstr: &Abstract<Chain>,
    interchain: &IBC,
    dest: &Abstract<Chain>,
    polytone_src: &Polytone<Chain>,
) -> Result<(), InterchainError> {
    // First we register client and host respectively
    let chain1_id = abstr.ibc.client.get_chain().chain_id();
    let chain1_name = ChainName::from_chain_id(&chain1_id);

    let chain2_id = dest.ibc.client.get_chain().chain_id();
    let chain2_name = ChainName::from_chain_id(&chain2_id);

    // First, we register the host with the client.
    // We register the polytone note with it because they are linked
    // This triggers an IBC message that is used to get back the proxy address
    let proxy_tx_result = abstr.ibc.client.register_infrastructure(
        chain2_name.to_string(),
        dest.ibc.host.address()?.to_string(),
        polytone_src.note.address()?.to_string(),
    )?;
    // We make sure the IBC execution is done so that the proxy address is saved inside the Abstract contract
    let _ = interchain.check_ibc(&chain1_id, proxy_tx_result)?;

    // Finally, we get the proxy address and register the proxy with the ibc host for the dest chain
    let proxy_address = abstr.ibc.client.host(chain2_name.to_string())?;

    dest.ibc.host.register_chain_proxy(
        chain1_name.to_string(),
        proxy_address.remote_polytone_proxy.unwrap(),
    )?;

    dest.account_factory.update_config(
        None,
        Some(dest.ibc.host.address()?.to_string()),
        None,
        None,
    )?;

    Ok(())
}
// #[test]
// fn successful_install() -> anyhow::Result<()> {
//     let mock = MockBech32::new("mock");
//     let env = TestEnv::setup(mock)?;
//     let app = env.app;

//     let config = app.config()?;
//     assert_eq!(config, ConfigResponse {});
//     Ok(())
// }

// #[test]
// fn successful_increment() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let app = env.app;

//     app.increment()?;
//     let count: CountResponse = app.count()?;
//     assert_eq!(count.count, 1);
//     Ok(())
// }

// #[test]
// fn successful_reset() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let app = env.app;

//     app.reset(42)?;
//     let count: CountResponse = app.count()?;
//     assert_eq!(count.count, 42);
//     Ok(())
// }

// #[test]
// fn failed_reset() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let app = env.app;

//     let err: UsbError = app
//         .call_as(&Addr::unchecked("NotAdmin"))
//         .reset(9)
//         .unwrap_err()
//         .downcast()
//         .unwrap();
//     assert_eq!(err, UsbError::Admin(AdminError::NotAdmin {}));
//     Ok(())
// }

// #[test]
// fn update_config() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let app = env.app;

//     app.update_config()?;
//     let config = app.config()?;
//     let expected_response = usb_plugin::msg::ConfigResponse {};
//     assert_eq!(config, expected_response);
//     Ok(())
// }

// #[test]
// fn balance_added() -> anyhow::Result<()> {
//     let env = TestEnv::setup()?;
//     let account = env.app.account();

//     // You can add balance to your account in test environment
//     let add_balance = coins(100, "ucosm");
//     account.add_balance(&add_balance)?;
//     let balances = account.query_balances()?;

//     assert_eq!(balances, add_balance);

//     // Or set balance to any other address using cw_orch
//     let mock_env = env.abs.environment();
//     mock_env.add_balance(&env.app.address()?, add_balance.clone())?;
//     let balances = mock_env.query_all_balances(&env.app.address()?)?;

//     assert_eq!(balances, add_balance);
//     Ok(())
// }
