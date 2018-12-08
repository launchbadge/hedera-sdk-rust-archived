use failure::Error;
use hedera::Client;
use hedera::SecretKey;
use std::env;

fn main() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;

    let client = Client::builder("testnet.hedera.com:50001")
        .operator(operator, operator_secret)
        .build()?;

    // Get the cost for getting the balance

    let balance_cost = client.account(operator).balance().cost()?;
    println!("cost:balance = {} tinybars", balance_cost);

    // Get _just_ the balance for the account first

    let balance = client.account(operator).balance().get()?;
    println!("balance = {} tinybars", balance);

    // Get the full information for the account
    // First we get how much this will cost

    let info_cost = client.account(operator).info().cost()?;
    println!("cost:info = {} tinybars", info_cost);

    // Now actually get the full information for the account

    let info = client.account(operator).info().get()?;
    println!("info = {:#?}", info);

    Ok(())
}
