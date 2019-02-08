#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{AccountId, Client, Status};
use std::{env, thread::sleep, time::Duration};
use tokio::{await, run_async};

// This example transfers crypto between the operator account and another

// to invoke from unix/macOs terminal
// export OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// export NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// export NODE_ACCOUNT=node's account (e.g. 0.0.3)
// export OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// export TRANSFER_TO=the account you wish to transfer to (e.g. 0.0.2)
// export TRANSFER_AMOUNT=the amount to transfer in tinybar (e.g. 10000)
// then from the hedera-sdk-rust root run:
// cargo run --example transfer_crypto

// to invoke from windows command line
// set OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// set NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// set NODE_ACCOUNT=node's account (e.g. 0.0.3)
// set OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// set TRANSFER_TO=the account you wish to transfer to (e.g. 0.0.2)
// set TRANSFER_AMOUNT=the amount to transfer in tinybar (e.g. 10000)
// then from the hedera-sdk-rust root run:
// cargo run --example transfer_crypto

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

    // Receiver is the account that receives the transferred crypto
    let receiver: AccountId = env::var("TRANSFER_TO")?.parse()?;

    let transfer_amount = env::var("TRANSFER_AMOUNT")?.parse::<i64>()?;
    // transfer tinybar from the operator account to the receiver account.
    let id = await!(client
        .transfer_crypto()
        .transfer(operator, -transfer_amount)
        .transfer(receiver, transfer_amount)
        .memo("[hedera-sdk-rust][example] transfer_crypto")
        .execute_async())?;
        // removed signatures for operator, they are added to the transaction automatically (transaction.rs)
        // .sign(&env::var("OPERATOR_SECRET")?.parse()?)
        // .sign(&env::var("OPERATOR_SECRET")?.parse()?)

    println!("created transfer; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(5));

    // Get the receipt and check the status to prove it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;

    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
