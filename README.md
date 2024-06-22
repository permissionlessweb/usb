# USB: Account Module To Interact With Jackal Protocol

The `usb-plugin` is a module app designed for accounts to interact with Jackal Protocol, in order to power an extremely convenient UX for users & developers who desire to save files with Jackal.

## Differnce Between Usb-Plugin & Usb-Adapter

### Usb-Plugin
The **Usb-Plugin** is instantiated for each Account individually and is migratable. Usb-Plugins is allowed to perform the actions as the Account, and can integrate with other Apps and Adapters installed on the Account. To learn more about Abstract Accounts, please see the [abstract accounts documentation](https://docs.abstract.money/3_framework/3_architecture.html). To read more about apps, please see the [app module documentation](https://docs.abstract.money/3_framework/6_module_types.html).


### Usb Adapter
Unlike The Usb-Plugin, the Usb-Adapter is shared between accounts. 

The **Usb-Adapter** serves as standard interface to extend the compatability with an Account and its storage options. The key function of an **Usb-Adapter** is to generalize functionality. **This currently is unimplemented**, however a goal for the adapter can be to generalize encryption schemes for various file storage methods. 

### Jackal Data Format Details

* **account -** `Hex[hash(Bech32 address)]`
* **rootHashPath -** `MerklePath("s")`
* **contents -** `FID`
* **editors -** 
    * c = `concatenate("e", trackingNumber, Bech32 address)`
    * map_key = `hex[hash("c")]`
    * map_value = `ECIES.encypt(aesIV + aesKey)`
* **viewers -**
    * c = `concatenate( "v", trackingNumber, Bech32 address )`
    * map_key = `hex[ hash("c") ]`
    * map_value = `ECIES.encrypt( aesIV + aesKey )`
* **trackingNumber -** `UUID used in viewers & editors map`
## Using the Justfile

This repository comes with a [`justfile`](https://github.com/casey/just), which is a handy task runner that helps with building, testing, and publishing your Abstract app module.

### Installing Tools

To fully make use of the `justfile`, you need to install a few tools first. You can do this by simply running `just install-tools`. See [tools used the template](https://docs.abstract.money/3_get_started/2_installation.html?#tools-used-in-the-template) for more information.

### Available Tasks

Here are some of the tasks available in the `justfile`:

- `install-tools`: Install all the tools needed to run the tasks.
- `wasm`: Optimize the contract.
- `test`: Run all tests.
- `fmt`: Format the codebase (including .toml).
- `lint`: Lint-check the codebase.
- `lintfix`: Fix linting errors automatically.
- `watch`: Watch the codebase and run `cargo check` on changes.
- `watch-test`: Watch the codebase and run tests on changes.
- `publish CHAIN_ID`: Publish the App to a network.
- `schema`: Generate the json schemas for the contract
<!-- - `ts-codegen`: Generate the typescript app code for the contract -->
<!-- - `ts-publish`: Publish the typescript app code to npm -->
- `publish-schemas`: Publish the schemas by creating a PR on the Abstract [schemas](https://github.com/AbstractSDK/schemas) repository.

You can see the full list of tasks available by running `just --list`.

### Compiling

You can compile your module(s) by running the following command:

```sh
just wasm
```

This should result in an artifacts directory being created in your project root. Inside you will find a `my_module.wasm` file that is your module’s binary.

### Testing

You can test the module using the different provided methods.

1. **Integration testing:** We provide an integration testing setup in both contracts. The App tests can be found here [here](./contracts/app/tests/integration.rs). You can re-use the setup provided in this file to test different execution and query entry-points of your module. Once you are satisfied with the results you can try publishing it to a real chain.
2. **Local Daemon (Optional):** Once you have confirmed that your module works as expected you can spin up a local node and deploy Abstract + your app onto the chain. You need [Docker](https://www.docker.com/) installed for this step. You can do this by running the [test-local](./contracts/app/examples/test-local.rs) example, which uses a locally running juno daemon to deploy to. You can setup local juno using `just juno-local` command. At this point you can also test your front-end with the contracts.

Once testing is done you can attempt an actual deployment on test and mainnet.

### Publishing

Before attempting to publish your app you need to add your mnemonic to the `.env` file. **Don't use a mnemonic that has mainnet funds for testing.**

<!-- It's also assumed that you have an account and module namespace claimed with this account before publishing. You can read how to do that [here](https://docs.abstract.money/4_get_started/5_abstract_client.html). -->
Select from a wide range of [supported chains](https://orchestrator.abstract.money/chains/index.html) before proceeding. Make sure you've some balance enough to pay gas for the transaction.

You can now use `just publish CHAIN_ID` to run the [`examples/publish.rs`](./examples/publish.rs) script. The script will publish the app to the networks that you provided. Make sure you have enough funds in your wallet on the different networks you aim to publish on.

### Publishing Module Schemas

To publish your module schemas, we provide the `publish-schemas` command, which creates a pull request on the Abstract [schemas](https://github.com/AbstractSDK/schemas) repository.

Please install [github cli](https://cli.github.com/) before proceeding. Also login and setup your github auth by `gh auth login`. Now, we're ready to proceed.

```bash
just publish-schemas <namespace> <name> <version>
```

- `namespace`: Your module's namespace(usb)
- `name`: Your module's name
- `version`: Your module's version. Note that if you only include the minor version (e.g., `0.1`), you don't have to reupload the schemas for every patch version.

The command will automatically clone the Abstract Schemas repository, create a new branch with the given namespace, name, and version, and copy the schemas and metadata from your module to the appropriate directory.

For this command to work properly, please make sure that your `metadata.json` file is located at the root of your module's directory. This file is necessary for the Abstract Frontend to correctly interpret and display information about your module.

Example:

```bash
just publish-schemas my-namespace my-module 0.0.1
```

In the example above, `my-namespace` is the namespace, `my-module` is the module's name, and `0.1` is the minor version. If you create a patch for your module (e.g., `0.1.1`), you don't need to run `publish-schemas` again unless the schemas have changed.


## Future Goals 
* Automate Storage Purchasing
* Manage Storage Provider 
* SubLease Storage 
* Send msgs locally if account is already on Jackal