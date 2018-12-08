use failure::Error;
use hedera::Client;
use hedera::SecretKey;
use std::env;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;
    let node = "0:0:3".parse()?;

    let client = Client::builder("testnet.hedera.com:50001")
        .operator(operator, operator_secret.clone())
        .node(node)
        .build()?;

    // Get the cost for getting the balance

    let balance_cost = client.account(operator).balance().cost()?;

    println!("cost:balance = {} tinybars", balance_cost);

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Get _just_ the balance for the account first

    let balance = client.account(operator).balance().get()?;

    println!("balance = {} tinybars", balance);
    println!("balance = {} hbars", (balance as f64) / 100000000.0);

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Get the full information for the account
    // First we get how much this will cost
    // Then we can manually construct a crypto transfer to the node to pay for our query

    let info_cost = client.account(operator).info().cost()?;

    println!("cost:info = {} tinybars", info_cost);

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Now actually get the full information for the account

    let info = client
        .account(operator)
        .info()
        .payment(
            client
                .transfer_crypto()
                .transfer(node, info_cost as i64)
                .transfer(operator, -(info_cost as i64)),
        )?
        .get()?;

    println!("info = {:#?}", info);

    Ok(())
}
