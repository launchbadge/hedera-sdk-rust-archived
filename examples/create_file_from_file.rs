#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};

// This example creates a new file with input from an existing file

// to invoke from unix/macOs terminal
// export OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// export NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// export NODE_ACCOUNT=node's account (e.g. 0.0.3)
// export OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// export FILE_PATH=path to the file to get the data from on your machine (e.g. examples/files/Hedera.txt), note this is relative to the hedera-sdk-rust root
// then from the hedera-sdk-rust root run:
// cargo run --example create_file_from_file

// to invoke from windows command line
// set OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// set NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// set NODE_ACCOUNT=node's account (e.g. 0.0.3)
// set OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// set FILE_PATH=path to the file to get the data from on your machine (e.g. examples/files/Hedera.txt), note this is relative to the hedera-sdk-rust root
// then from the hedera-sdk-rust root run:
// cargo run --example create_file_from_file

async fn main_() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = env::var("OPERATOR")?.parse()?;
    let node_port : String = env::var("NODE_PORT")?;
    let client = Client::builder(&node_port)
        .node(env::var("NODE_ACCOUNT")?.parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    let operator_secret : String = env::var("OPERATOR_SECRET")?;
    let secret = SecretKey::from_str(&operator_secret)?;
    let public = secret.public();

    // load file from file system
    let file_contents = std::fs::read(env::var("FILE_PATH")?)?;

    // Create a file
    let id = await!(client
        .create_file()
        .expires_in(Duration::from_secs(2_592_000))
        .key(public)
        .contents(file_contents)
        .memo("[hedera-sdk-rust][example] create_file")
        .sign(&env::var("OPERATOR_SECRET")?.parse()?) // sign as the owner of the file
        .execute_async())?;

    println!("creating file; transaction = {}", id);

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

    let file = receipt.file_id.unwrap();
    println!("file ID = {}", file);
    println!("Run these (OS Depending) to run further file examples");
    println!("export FILE_ID={}", file);
    println!("set FILE_ID={}", file);

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
