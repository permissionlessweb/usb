use abstract_cw_orch_polytone::Polytone;
use abstract_interface::{
    Abstract, AbstractAccount, AccountDetails, AccountFactoryExecFns, ManagerQueryFns,
};
// Use prelude to get all the necessary imports
use cw_orch::{anyhow::Result, contract::Deploy, prelude::*};
use cw_orch_interchain::{
    IbcQueryHandler, InterchainEnv, InterchainError, MockBech32InterchainEnv,
};
use polytone::handshake::POLYTONE_VERSION;
use usb_plugin::{
    contract::interface::UsbInterface,
    msg::{UsbExecuteMsgFns, UsbInstantiateMsg, UsbQueryMsgFns},
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

struct TestEnv<Env: CwEnv> {
    env: Env,
    abs: AbstractClient<Env>,
    client1: Application<Env, UsbInterface<Env>>,
    client2: Application<Env, UsbInterface<Env>>,
}

pub const TEST_ACCOUNT_NAME: &str = "account-test";
pub const TEST_ACCOUNT_DESCRIPTION: &str = "Description of an account";
pub const TEST_ACCOUNT_LINK: &str = "https://google.com";

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
        // Install USB Module
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

        Ok(TestEnv {
            env,
            abs: abs_client,
            client1: app,
            client2: app2,
        })
    }

    // deploy polytone contracts.
    fn enable_ibc(&self) -> Result<()> {
        Polytone::deploy_on(self.abs.environment().clone(), None)?;
        Ok(())
    }
}

mod basic_functions {
    use usb::JackalMsg;

    use super::*;

    // Jackal storage encryption workflow
    // 1. generate random key offline
    // 2. encrypt file with key
    // 3. encrypt key with wallet pubkey & signature, store to x/filetree
    #[test]
    fn save_file() -> Result<()> {
        // a. Create a sender and mock env for each chain
        let interchain = MockBech32InterchainEnv::new(vec![
            ("juno-1", "juno1fxccvvhhy43tvet2ah7jqwq4cwl9k3dx2kyce9"),
            ("jackal-1", "jkl1tyl97ac3s7sec4jwznk0s7n3tlwf3math03qj4"),
        ]);
        // b. Create Test Environments
        let bs_env = TestEnv::setup(interchain.chain("juno-1")?)?;
        let jkl_env = TestEnv::setup(interchain.chain("jackal-1")?)?;

        // c. Enable IBC
        bs_env.enable_ibc()?;
        jkl_env.enable_ibc()?;

        // d. Connect Chains To Each Other
        ibc_connect_polytone_and_abstract(&interchain, "juno-1", "jackal-1")?;

        let bs_client = bs_env.client1;
        let jkl_client = jkl_env.client1;

        bs_client.account().set_ibc_status(true)?;

        let msg = JackalMsg::MakeRoot {
            editors: "test".to_string(),
            viewers: "test".to_string(),
            tracking_number: "test".to_string(),
        };

        bs_client.jackal_msgs(vec![msg])?;

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
