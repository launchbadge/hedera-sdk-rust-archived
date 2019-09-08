# Hedera SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/hedera.svg?style=popout-square)](https://crates.io/crates/hedera)
[![License](https://img.shields.io/crates/l/hedera.svg?style=popout-square)](https://github.com/hashgraph/hedera-sdk-rust/blob/master/LICENSE.txt)

This repo contains the Rust SDK for interacting with the [Hedera](https://hedera.com) platform. Hedera is the _only_ **public** distributed ledger licensed to use the Hashgraph consensus algorithm for fast, fair and secure transactions. By using any of the Hedera SDKs, developers will be empowered to build an entirely new class of decentralized applications.

Following the instructions below should help you to reach the position where you can send transactions and queries to a Hedera testnet or the Hedera mainnet.

## Table of Contents

* **[Developer Rewards][01-dev-rewards]**
* **[Architectural Overview][02-arch-overview]**
* **[Prerequisites][03-prerequisites]**
  * **[Software][03-01-software]**
  * **[Hedera Account][03-02-hedera-acct]**
  * **[Hedera testnet access][03-03-hedera-testnet]**
* **[Installing the Hedera SDK for Rust][04-installing]**
  * **[From an existing clone of this SDK repo][04-01-from-clone]**
  * **[Building a new project for your Hedera Rust app][04-02-new-project]**
  * **[Running the examples][04-03-run-examples]**
* **[Creating a public/private keypair for testnet use][05-create-keypair]**
* **[Associating your public key with you Hedera testnet account][06-assoc-key]**
* **[Your First Hedera Application][07-first-hedera]**
  * **[Checking your account balance][07-01-check-balance]**
  * **[Enhance your application to check a friend's account balance][07-02-friend-balance]**
  * **[Next step: Transferring hbars to a friend's account][07-03-transfer-hbars]**
* **[Other resources][08-other-res]**
* **[Getting in touch][09-get-in-touch]**
* **[Contributing to this project][10-contribute]**
* **[License information][11-license]**

--------

## Developer Rewards

Developers who create Hedera accounts and test their applications on our testnet will be offered the opportunity to earn real mainnet __ℏ__ ([__hbars__](#a-hbar)) based upon that testing activity. The SDKs are intended to facilitate application development and encourage such testing. See the [Hedera Account][03-02-hedera-acct] section for further details.

> <a id="a-hbar">**Hbars**:</a>
>
> _Hbars_ are the native cryptocurrency that is used to pay for transactions on the Hedera platform and to secure the network from certain types of cyberattacks. They are the native platform coin needed to interact with and exchange value on Hedera.
>
> The symbol for hbars is "**ℏ**" so `5 ℏ` means 5 hbars
>
> <a id="a-tinybar">**Tinybars**:</a>
>
> Tinybars are (not surprisingly) smaller than hbars. They are used to divide hbars into smaller amounts. One hbar is equivalent to one hundred million tinybars.
>
> The symbol for tinybars is "**tℏ**" so it is correct to say `1 ℏ = 100,000,000 tℏ`
>
> _**Important Note**: The values of all fees and transfers throughout the Hedera SDKs are represented in tinybars, though the term hbars may be used for the purposes of brevity._

## Architectural Overview

All Hedera SDKs are intended to provide a developer-friendly means to leverage the Hedera API, which is based on [Google Protobuf](https://developers.google.com/protocol-buffers/). Protobuf supports code generation for a growing number of languages and is highly optimised for efficient communications. For those interesting in viewing the underlying protobuf message definitions, see the [Hedera Protobuf Message Definitions](https://github.com/hashgraph/hedera-protobuf) repo.

Developers who wish to work in other languages are at liberty to do so, but should be aware that implementation of cryptographic key generation and manipulation is not a trivial undertaking. The source code for the cryptography libraries used by all other Hedera SDKs (except Java) use C libraries generated from the Rust code in this repository. We would recommend use of these same libraries for developers interested in adding support for other languages.

## Prerequisites

### Software

* [Rust](https://www.rust-lang.org/) – can be downloaded using [these instructions](https://www.rust-lang.org/tools/install).
  * Where possible, use of `rustup` is highly recommended as it facilitates version and dependency management.
  * An introduction to the Rust programming language can be found [here](https://www.rust-lang.org/learn).
  * To confirm that you have successfully installed Rust and Cargo (the Rust build tool and package manager) the following command can be executed from a terminal window:

    ```sh
    cargo --version
    ```

    You should see version `1.32.0` or higher. If this command fails, the most likely reason relates to the `PATH` environment variable as explained in [the instructions](https://www.rust-lang.org/tools/install).

  * The `rust-toolchain` file in this repo currently makes this project dependent on a nightly build of Rust. Users who choose manage this project using `rustup` should not notice this, as the correct version of Rust will be installed transparently as required. This dependency will be removed when the asynchronous I/O packages used by this SDK are deemed stable, which is expected to happen in March 2019.

* [Protobuf](https://developers.google.com/protocol-buffers/)
  * The Hedera Rust SDK implementation relies upon on Google [Protobuf](https://developers.google.com/protocol-buffers/) to generate portions of the Rust source code for the SDK.  To compile the SDK, the protobuf compiler, `protoc`, must be installed and available to the system.  To confirm that protobuf compiler is available: execute the following command from a terminal window:

    ```sh
    protoc --version
    ```

    You should see version `3.3.x` or later.  If the command fails, it is likely that the `protoc` executable is not included in the `PATH` environment variable, or perhaps absent from your system.  It can be installed as explained in the [readme](https://github.com/protocolbuffers/protobuf).

### Hedera Account

The [Hedera Portal](https://go.hedera.com) allows people to create a **Hedera Account** facilitating access to the Hedera mainnet and Hedera testnets. A Hedera Account allows entry of a testnet access code, in order to add a number of testnet __ℏ__ ([hbars](#a-hbar)) to a testnet account created (as can be seen [below][03-03-hedera-testnet]) using Hedera SDKs. The public key associated with a testnet account must also be associated with your Hedera account. A detailed explanation of this whole process is contained within this document.

* In order to gain early access (before Open Access) to a Hedera testnet or the Hedera mainnet users must create a Hedera account, including full identity verification. You can do this using the [Hedera Portal](https://go.hedera.com).
* We want to allow devs to earn __ℏ__ ([hbars](#a-hbar)) by helping us to test our SDKs. Hedera is based in the USA, so for us to be allowed to do this under US law we need to verify your identity as a part of the account creation process.

A full explanation of the Portal, Hedera accounts, identity verification and many other topics can be found at [Hedera Help](https://help.hedera.com). New users should head for the [Getting Started](https://help.hedera.com/hc/en-us/categories/360000099938-Getting-Started) section.

### Hedera testnet access

A Hedera testnet provides a test environment for testing your code without having to spend "real" mainnet __ℏ__ ([hbars](#a-hbar)). Testnet hbars are akin to "monopoly money" and have no intrinsic value, but testing against testnets will help you **earn** real **mainnet ℏ ([hbars](#a-hbar))**. It is worth noting that the virtual infrastructure used to provide testnets is not intended for performance testing, as the specification of nodes is not in any way equivalent to that of mainnet nodes. Further information on this topic is included within the "Testnet Performance and Throttling" section further on in these instructions.

* Once you have your Hedera account set up, you can request access to Hedera test networks by filling out the form [here](https://learn.hedera.com/HederaTestnetAccess).
* Check for answers to your testnet-related questions in the *Testnet Activation* section on [this](https://help.hedera.com/hc/en-us/categories/360000099938-Getting-Started) page.
* Please note that there is a waiting list for testnet access; please be patient. If you haven't had a response withing 10 days, reach out to to the team on [discord](https://hedera.com/discord).
* Once you have received a testnet access code, enter it into the testnet access box and push the proceed/arrow button. You should see a message "_Access code confirmed. By activating this code you will switch to testnetXXX with a starting value of 1000 hbars._" Please note that these are **testnet** hbars and should not to be confused with mainnet hbars.
* Pushing the "Activate and switch network" button will credit your testnet account with those testnet hbars and switch the portal to the testnet. You can switch the portal between mainnet and testnet at any time by using the drop-down at the top of the page.

## Installing the Hedera SDK for Rust

This SDK can be run be cloning the SDK or by creating a new project folder that includes a dependency on the Hedera SDK for Rust. The examples in the "Building your first Hedera application" section later in this document the latter (new project) approach will be used.

### From an existing clone of this SDK repo

* For those who have already cloned this github repo, running the following command from the root folder of the SDK in a terminal window should be sufficient to retrieve all required packages:

  ```sh
  cargo build
  ```

* If you're having trouble, please review the [Prerequisites][03-prerequisites] section.

### Building a new project for your Hedera Rust app

You can use the Hedera SDK for Rust from your own project without the need to clone the github repo.

* To set up a new skeletal project in a new folder, run the following commands from a terminal window, replacing `hello-future` with the name of the project (folder) you want to create:

  ```sh
  cargo install cargo-edit
  cargo new hello-future
  cargo add hedera
  ```

* `cargo-edit` is a tool that extends `cargo` to allow command-line manipulation of the `Cargo.toml` file. See [here](https://crates.io/crates/cargo-edit) for more information.

* Copy the `rust-toolchain` file from this repository into the new project folder. This ensures that the latest nightly build of Rust is used by your project. Explanation of this requirement can be found in the [Prerequisites][03-prerequisites] section.

* Add your own code to the `/src/main.rs` file in your new project folder.

* The examples in this document use the `failure` crate (see details [here](https://crates.io/crates/failure)) for error handling. To add this dependency to your project from a terminal window, run the following command:

  ```sh
  cargo add failure
  ```

### Running the examples

After running `cargo build` it should be possible to run the examples contained within this repo. In most cases, doing so will require changes to those examples to make them work with _your_ account on _your_ testnet. Detailed explanations of such changes can be seen in subsequent sections of this document.

If you have already modified the examples accordingly, running the following command from a terminal window will execute the example. Make sure you replace `<filename>` with the name (e.g. generate_key) of one of the example files. The `.rs` suffix is not required.

```sh
cargo run --example <filename>
```

## Creating a public/private keypair for testnet use

As a general principle, it is bad practice to use your mainnet keys on a testnet. The code below shows the content of the [generate_key example](/examples/generate_key.rs) file. This shows how you can create new public and private keys using the Hedera SDK for Rust:

```rust
use hedera::SecretKey;

fn main() {
    let (secret, mnemonic) = SecretKey::generate("");
    let public = secret.public();

    println!("secret   = {}", secret);
    println!("mnemonic = {}", mnemonic);
    println!("public   = {}", public);
}
```

* It can be executed from a terminal window from the root folder of this repo by and typing:

```sh
cargo run --example generate_key
```

* Make careful note of the 24-word mnemonic and both of the keys that are generated. For a testnet you can copy and paste this information into a text file. For security reasons you should **_never do this for mainnet_**.

> **Development Environment Note**
>
> All of the commands described in these instructions can also be executed using an IDE – such as [VSCode](https://code.visualstudio.com/) or [IntelliJ IDEA](https://www.jetbrains.com/idea) (with [Rust plugin](https://intellij-rust.github.io/)), or editors such as [Atom](https://atom.io/) – according to each developer's preferences and budget. _Hedera has no affiliation with the companies providing these or other equivalent tools._
>
> Throughout these instructions you'll find the phrase "Run the following command from a terminal window." Feel free to use your IDE whenever you see this – if that's how you prefer to work. Terminal is used in this document to avoid ambiguity.

## Associating your public key with you Hedera tesnet account

Once you have generated a public/private keypair as described [above][05-create-keypair], you need to link the **public** key to your Hedera testnet account. To do this, return to the Hedera portal and ensure that you have the testnet selected in the drop-down at the top of the page.

You should see the box asking you to `Enter your Public Key`. Copy the long hex value of the **public** key and paste it into that textbox in the portal. Do make sure you that you select all of the characters. Click `Submit`.

You should briefly see an "Account Pending" message, which will be replaced with an Account ID box. You should make a note of your account ID - perhaps in the same text file where you have stored your public and private testnet keys.

All Hedera IDs consist of three numbers (`int64s`) separated by the "`.`" symbol. The three numbers represent `Shard number`, `Realm number` and `Account number` respectively. Shards and Realms are not yet in use so expect the first two numbers to be zeros.

You should also scroll down until you see a box labelled "Network" and make a note of the `Address`, which will be something like `testnet.hedera.com:50222`. You should also take note of the `Node`, which represents the account ID of a node on the testnet; it will be something like `0.0.3`.

## Your First Hedera Application

### Checking your account balance

A more complete example, which includes code fragments in this section can be found in the [get_account example](/examples/get_account.rs) file located in the [examples](/examples/) folder of this repo. This simplified example is broken into bite-sized pieces here so that accompanying explanations can be seen in context alongside each fragment.

This explanation assumes that a "hello_future" (or equivalent) project has been created as explained in the "Building a new project for your Hedera Rust app" instructions earlier in this document. The following code samples are intended to represent the contents of the `src/main.rs` file therein.

Firstly the `failure::Error` and `hedera::Client` crates are imported.

The `std::thread:sleep` and `std::time::Duration` are also imported but commented out for now. These crates will be needed later in this example and can be un-commented when required by removing the preceding `//`. Uncommenting this import before the crates are used will result in an "`unused import`" warning when the code is run.

It's also useful to create a constant `ONE_HBAR` to represent the number of **_[tinybars](#a-tinybar)_** in one **_[hbar](#a-hbar)_**:

```rust
use failure::Error;
use hedera::Client;
//use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Error> {

  const ONE_HBAR: i64 = 100_000_000;
```

Create and set a `my_account` variable, replacing `1234` with your own `Account ID` from the portal. This is the account for which we will retrieve the balance.

```rust
  let my_account: String = "0.0.1234".parse()?;
```

All Hedera transactions and queries must be sent to a Hedera node. This can be specified using a `node_account` variable. Testnets provide each developer with a single node's account ID (labelled `node` in the portal) in order to simplify this and limit testnet infrastructure burden. On mainnet it will be the responsibility of the application to choose a node - usually at random.

The node account ID should look something like `0.0.3`.You should use the ID you see in the portal in the following code:

```rust
  let node_account: String = "0.0.3".parse()?;
```

> **Node Account Defaults**: When using testnets, it is worth noting that the SDK uses node account `0.0.3` by default – even when no node account is specified.

It is also important to specify which account is initiating this query – known as the **operator** account. In this case the operator account is the same account for which the balance is to be checked, so the same `my_account` variable can be used.

```rust
  let operator = my_account.parse()?;
```

Next, you need to establish a connection to the Hedera testnet using the `Address` you noted earlier from the network section shown on the Hedera portal. Make sure that you replace `testnet.hedera.com:50222` with the equivalent address you copied from the portal.

Whilst establishing the connection to Hedera the **private** key of the  operator account – in this case your account, so your private key – can be specified. This is required in order to authorise the payment of a small fee for the execution of this query. Be sure to replace `<my-private-key>` with the private key you generated near the start of these instructions.

```rust
  let client = Client::builder("testnet.hedera.com:50222")
    .node(node_account.parse()?)
    .operator(operator, || "<my-private-key>")
    .build()?;
```

> **Security Tip**: In the [get_account_balance example](/examples/get_account.rs) file located in the [examples](/examples/) folder, the `env::var("OPERATOR_SECRET")` function is used. This retrieves the private key from an environment variable called OPERATOR_SECRET. It is good practice to use this technique, as it avoids accidental publication of private keys when storing code in public repos or sharing them accidentally via collaboration tools.
>
> **Don't forget**: If someone else knows your private key, they effectively own your account! Although the impact of this is low when using testnets, it could be a very expensive mistake on mainnet. For purposes of clarity, the example code above has been simplified and does not use an environment variable.

At this point, you're ready to query your account balance. The `client.account(operator).balance()` constructs the request; adding `.get()` executes that request.

You can the output the balance using `println!` macro and end the program indicating success with `Ok(())` and then closing the braces for `fn main`.

For illustrative purposes, we're showing the balance in **_[tinybars](#a-tinybar)_** and **_[hbars](#a-hbar)_**. The Hedera SDKs represent all quantities for transfers and fees as integers using _tinybars_. There are one hundred million (100,000,000) tinybars in one hbar.

```rust
  let my_tinybars = client.account(operator).balance().get()?;
  let my_hbars: f64 = my_tinybars as f64 / ONE_HBAR as f64;
  println!("Account {} balance = {} tinybars", my_account, my_tinybars);
  println!("Account {} balance = {} hbars", my_account, my_hbars);

  Ok(())
}
```

You should now be able to **run your first Hedera program** by executing `cargo run main.rs` from terminal.

If everything went according to plan you should see something like this:

```sh
Account 1234 balance = 100500005000 tinybars
Account 1234 balance = 1005.00005 hbars
```

#### Testnet performance and throttling

> For the present, our testnets have been throttled, allowing a limited number of Hedera transactions per account per second. We're using virtual infrastructure to support the huge demand we have had for testnet access, and prefer to foster innovative use of these resources and discourage folks from trying to generate metrics using underspecified hardware.
>
> If you see error messages like `transaction failed the pre-check: Busy`, it is likely that you are exceeding these throttling thresholds. To avoid such errors, short delays can be added. To add a one second delay, for example, use the following code between transactions or queries:
>
>```rust
>  sleep(Duration::from_secs(1));
>```
>
> Don't forget to remove the `//` comment symbols before the `use std:{....},` statement near the beginning of the program. This was disabled to prevent the "`unused import`" warning caused when the code is run before those crates are used in the code.

### Enhance your application to check a friend's account balance

If you know the account ID of another account on your testnet – perhaps a friend or colleague – you can also check their balance.

> **Creating an additional testnet account**
>
> If your friends won’t share their accounts, or if you don’t have any friends, see the [Create Account Example](/examples/create_account.rs) included in this repo.
>
> If you do choose to create an account using that example, don't forget to do the following:
>
> 1. Create a local environment variable OPERATOR_SECRET that contains your private key.
> 2. Make sure that the node account ID matches the ID you see in the portal.
> 3. Update the `testnet.hedera.com:...` testnet address to the correct one.
> 4. Change the `operator` variable value to your own testnet account ID.
> 5. Change the `initial_balance` to an acceptable quantity of testnet tinybars.

* For the purposes of this example, an Account ID of `0.0.1235` will be used for that second account. Don't forget to amend `1235` to the account number of your friend's account. If you forget to do this will you will probably see a `transaction failed the pre-check: InvalidAccount` message.

* To continue with this example, add the next code code block into your existing `main.rs` file, just before the `Ok(())` statement.

* As mentioned above, we will add a small delay to ensure that we do not exceed testnet throttling limits. For brevity, this statement will be included _without further comment_ in all subsequent examples. If you get an error here, you need to remove the `//` comment symbols before the `use std:{....},` statement near the beginning of the program.

* Before executing any transfers, you can initialise a second variable `friend_account` representing the second account, query its balance and output the result.

```rust
  sleep(Duration::from_secs(1));

  let friend_account: String = "0.0.1235".parse()?;

  let friend = friend_account.parse()?;

  let friend_tinybars = client.account(friend).balance().get()?;
  let friend_hbars: f64 = friend_tinybars as f64 / ONE_HBAR as f64;
  println!("Account {} balance = {} tinybars", friend_account, friend_tinybars);
  println!("Account {} balance = {} hbars", friend_account, friend_hbars);
```

* Run the program again by executing `cargo run main.rs` from terminal.

* You should see your balance followed by your friend's balance.

> **Transaction and Query Fees**
>
> Note that your balance will decrease slightly each time you execute your code. This is due to the small fees associated with each query or transaction on the Hedera platform. On a testnet, this is not all that important, but it's worth keeping on mind when using mainnet.

### Next step: Transferring _[hbars](#a-hbar)_ to a friend's account

* A `transfer_amount` variable can be used to make the next steps more readable. In this case, we'll transfer **10 ℏ** and output details of the intended transaction.

```rust
let transfer_amount: i64 = 10 * ONE_HBAR;
  println!("Starting transfer of {} tinybars from Account {} to Account {}", transfer_amount, my_account, friend_account);
```

* It is worth re-stating that a __secret__ key (also known as _private_ key) is required in order to transfer _hbars_ from an account. Since the operator private key has already been set for this client session, it is not necessary to sign this transaction explicitly.

* The next statement is a little more complex so each line is explained individually below the code.

```rust
  let transaction_id = client
    .transfer_crypto()
    .transfer(operator, -1 * transfer_amount)
    .transfer(friend, transfer_amount)
    .memo("My first transfer of hbars! w00t!")
    .execute()?;
```

#### Explanation of the above code block by line number

__1__. `let transaction_id = client` declares a `transaction_id` variable and populates it with the result of this transaction.

__2__. `transfer_crypto()` specifies that the transaction will transfer **_hbars_** between accounts.

__3__. `transfer(operator, -1 * transfer_amount)` sets up part of the transfer. In this case _from_ **your** account. Note that the `* -1` makes the amount negative, denoting that **_hbars_** will be **deducted** from this account.

__4__. `transfer(friend, transfer_amount)` sets up the second part of this transfer. In this case _to_ your **friend's**  account. A positive number indicates that this account will be **incremented** by the specified amount.

__5__. `memo("My first transfer of hbars! w00t!")` assigns a label to the transaction of up to 100 bytes. Use of this field is at the developer's discretion and does not affect the behaviour of the plaform.

__6__. `execute()?;` executes the transaction.

> #### Multi-party transfers
>
> It is possible to create a transfer transaction containing **multiple** _to_ and **multiple** _from_accounts within that same transaction. In a case where multiple accounts were to be debited, signatures would be required for each one, and additional `.sign(...)` lines would have to be added.
>
> __Important__: the _sum of all amounts_ in `.transfer(...)` lines contained within in a `transfer_crypto` transaction _**must** add up to **zero**_.

* The `transaction_id_` variable should now contain a reference to this transfer transaction. A transaction ID is made up of the account ID and the transaction timestamp – right down to nanoseconds.

* It makes sense to wait a little longer (2 seconds) after sending the transaction, so that the Hedera network can reach consensus on the transaction.

```rust
  println!("Transaction sent. Transaction ID is {}", transaction_id);

  sleep(Duration::from_secs(2));
```

* To confirm that the transaction succeeded a `receipt` can be requested, so we can define a corresponding variable. Although this is not a mandatory step, it does verify that this transaction successfully reached network consensus.

```rust
  let receipt = client.transaction(transaction_id).receipt().get()?;
  if receipt.status == Status::Success {
    println!("Transaction Successful. Consensus confirmed.");
  } else {
    Err(format_err!(
      "Transaction unsuccessful. Status: {:?}",
      receipt.status
    ))?;
  }
```

* Finally, the balances of both accounts can be requeried to verify that the **10 ℏ** was indeed transferred from your account to that of your friend.

```rust
  let my_tinybars = client.account(operator).balance().get()?;
  let my_hbars: f64 = my_tinybars as f64 / ONE_HBAR as f64;
  println!("Account {} balance = {} tinybars", my_account, my_tinybars);
  println!("Account {} balance = {} hbars", my_account, my_hbars);

  let friend_tinybars = client.account(friend).balance().get()?;
  let friend_hbars: f64 = friend_tinybars as f64 / ONE_HBAR as f64;
  println!("Account {} balance = {} tinybars", friend_account, friend_tinybars);
  println!("Account {} balance = {} hbars", friend_account, friend_hbars);
```

* Run the program again by executing `cargo run main.rs` from terminal.

* You should now see both balances prior to the transfer followed by details of the transfer including success/failure. You should then be able to see the balances of both accounts after the transfer, demonstrating that **10 ℏ** has been transferred from your account to you friend's account. Hopefully it looks something like this:

```txt
Account 1234 balance = 96495305000 tinybars
Account 1234 balance = 964.95305 hbars
Account 1235 balance = 4000000000 tinybars
Account 1235 balance = 40.00000 hbars
Transfering 1000000000 tinybars from Account 1234 to Account 1235
Transfer Sent. Transaction ID is 0:0:1234@1548679850.429332000
Transaction Successful. Consensus confirmed.
Account 1234 balance = 95494805000 tinybars
Account 1234 balance = 954.94805 hbars
Account 1235 balance = 5000000000 tinybars
Account 1235 balance = 50.00000 hbars
```

## Other resources

* For an explanation of the underlying hashgraph algorithm, please consult our [whitepaper](https://www.hedera.com/hh-whitepaper-v1.4-181017.pdf) or Dr. Leemon Baird's 52-minute [Simple Explanation](https://www.youtube.com/watch?v=wgwYU1Zr9Tg) video.
* Links to all Hedera news and information can be found in at [Hedera Help](https://help.hedera.com) – including Coq validation of the hashgraph ABFT algorithm.
* 300+ [Hedera interviews and videos](https://www.youtube.com/watch?v=v2M0eo9PRxw&list=PLuVX2ncHNKCwe1BdF6GH6RnjrF7J7yTZ4) on YouTube. Thanks to Arvydas – a Hedera MVP – for curating this list.

## Getting in touch

Please reach out to us on the Hedera [discord channels](https://hedera.com/discord). We're fortunate to have an active community of over 5000 like-minded devs, who are passionate about our tech. The Hedera Developer Advocacy team also participates actively.

## Contributing to this Project

We welcome participation from all developers! For instructions on how to contribute to this repo, please review the [Contributing Guide](CONTRIBUTING.md).

## License Information

Licensed under Apache License,
Version 2.0 – see [LICENSE](LICENSE) in this repo or [apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)

[//]: # (Internal reference links)
[01-dev-rewards]: #developer-rewards
[02-arch-overview]: #architectural-overview
[03-prerequisites]: #prerequisites
[03-01-software]: #software
[03-02-hedera-acct]: #hedera-account
[03-03-hedera-testnet]: #hedera-testnet-access
[04-installing]: #installing-the-hedera-sdk-for-rust
[04-01-from-clone]: #from-an-existing-clone-of-this-sdk-repo
[04-02-new-project]: #building-a-new-project-for-your-hedera-rust-app
[04-03-run-examples]: #running-the-examples
[05-create-keypair]: #creating-a-publicprivate-keypair-for-testnet-use
[06-assoc-key]: #associating-your-public-key-with-you-hedera-tesnet-account
[07-first-hedera]: #your-first-hedera-application
[07-01-check-balance]: #checking-your-account-balance
[07-02-friend-balance]: #enhance-your-application-to-check-a-friends-account-balance
[07-03-transfer-hbars]: #next-step-transferring-hbars-to-a-friends-account
[08-other-res]: #other-resources
[09-get-in-touch]: #getting-in-touch
[10-contribute]: #contributing-to-this-project
[11-license]: #license-information
