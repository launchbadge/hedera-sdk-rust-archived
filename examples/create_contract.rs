#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};

// This example creates a new smart contract from an existing Hedera file

// to invoke from unix/macOs terminal
// export OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// export NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// export NODE_ACCOUNT=node's account (e.g. 0.0.3)
// export OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// export FILE_ID=Hedera file ID containing the smart contract byte code (e.g. 0.0.1032)
// export GAS=gas limit for creating the smart contract in tinybar (e.g. 1000000)
// then from the hedera-sdk-rust root run:
// cargo run --example create_contract

// to invoke from windows command line
// set OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// set NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// set NODE_ACCOUNT=node's account (e.g. 0.0.3)
// set OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// set FILE_ID=Hedera file ID containing the smart contract byte code (e.g. 0.0.1032)
// set GAS=gas limit for creating the smart contract in tinybar (e.g. 1000000)
// then from the hedera-sdk-rust root run:
// cargo run --example create_contract

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
    let file_id = env::var("FILE_ID")?.parse()?;
    let gas = env::var("GAS")?.parse::<i64>()?;

    // Create a contract from an existing Hedera file
    let id = await!(client
        .create_contract()
        .file(file_id)
        .gas(gas)
        .auto_renew_period(Duration::from_secs(2_592_000))
        .memo("[hedera-sdk-rust][example] create_contract")
        .execute_async())?;

    println!("creating contract; transaction = {}", id);

        //.constructor_parameters(params: Vec<u8>)
        // .initial_balance(balance: i64)
        // .sign(&env::var("OPERATOR_SECRET")?.parse()?) // sign as the owner of the file

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

    let contract = receipt.contract_id.unwrap();
    
    println!("contract ID = {}", contract);
    println!("Run these (OS Depending) to run further contract examples");
    println!("export CONTRACT_ID={}", contract);
    println!("set CONTRACT_ID={}", contract);

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
