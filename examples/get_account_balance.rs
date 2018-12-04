use failure::Error;
use hedera::Client;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let target = "0:0:2".parse()?;

    let client = Client::builder("testnet.hedera.com:50001").build()?;


    // Get the _cost_ or transaction fee for the query of getting the account balance
    let cost = client.account(target).balance().cost()?;

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Get the _answer_ for the query of getting the account balance (which is the actual balance)
    let balance = client.account(target).balance().get()?;

    println!("cost    = {} tinybars", cost);
    println!("balance = {} tinybars", balance);
    println!("balance = {} hbars", (balance as f64) / 100000000.0);

    Ok(())
}
