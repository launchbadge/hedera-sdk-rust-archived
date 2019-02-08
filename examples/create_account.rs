#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use tokio::{await, run_async};

// This example creates a new account

// to invoke from unix/macOs terminal
// export OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// export NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// export NODE_ACCOUNT=node's account (e.g. 0.0.3)
// export OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// then from the hedera-sdk-rust root run:
// cargo run --example create_account

// to invoke from windows command line
// set OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// set NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// set NODE_ACCOUNT=node's account (e.g. 0.0.3)
// set OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// then from the hedera-sdk-rust root run:
// cargo run --example create_account


async fn main_() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let (secret, _) = SecretKey::generate("");
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = env::var("OPERATOR")?.parse()?;
    let node_port : String = env::var("NODE_PORT")?;
    let client = Client::builder(&node_port)
        .node(env::var("NODE_ACCOUNT")?.parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Create our account
    let id = await!(client
        .create_account()
        .key(public)
        .initial_balance(5_000_000)
        .memo("[hedera-sdk-rust][example] create_account")
        .execute_async())?;

    println!("created account; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;
    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    // note: account can be [None] if the receipt wasn't for creating an account
    let account = receipt.account_id.unwrap();
    println!("account = {}", account);
    println!("Run these (OS Depending) to run further account examples");
    println!("export ACCOUNT_ID={}", account);
    println!("set ACCOUNT_ID={}", account);

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
