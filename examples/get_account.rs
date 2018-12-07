use failure::Error;
use hedera::Client;
use std::{thread::sleep, time::Duration};
use std::env;

fn main() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret= env::var("OPERATOR_SECRET")?.parse()?;
    let target = "0:0:2".parse()?;
    let node = "0:0:3".parse()?;

    let client = Client::new("testnet.hedera.com:50001")?;

    // Get _just_ the balance for the account first

    let balance = client.account(target).balance().get()?;

    println!("balance = {} tinybars", balance);
    println!("balance = {} hbars", (balance as f64) / 100000000.0);

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Get the full information for the account
    // First we get how much this will cost
    // Then we can manually construct a crypto transfer to the node to pay for our query

    let info_cost = client.account(target).info().cost()?;

    println!("info cost = {} tinybars", info_cost);

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Now actually get the full information for the account

    let info = client.account(target).info()
        .payment(client.transfer_crypto()
            .operator(operator)
            .node(node)
            .transfer(node, info_cost as i64)
            .transfer(operator, -(info_cost  as i64))
            .sign(&operator_secret)
            .sign(&operator_secret))?
        .get()?;

    println!("info = {:#?}", info);

    Ok(())
}
