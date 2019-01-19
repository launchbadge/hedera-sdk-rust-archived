use failure::Error;
use hedera::Client;
use std::env;

fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let operator = env::var("OPERATOR")?.parse()?;
    let client = Client::builder("testnet.hedera.com:50131")
        .node("0:0:3".parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar

    let balance = client.account(operator).balance().get()?;
    println!("balance = {} tinybars", balance);

    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    let info = client.account(operator).info().get()?;
    println!("info = {:#?}", info);

    Ok(())
}
