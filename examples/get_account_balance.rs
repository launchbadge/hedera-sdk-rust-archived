use failure::Error;
use hedera::{Client, QueryGetAccountBalance};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Error> {
    let client = Client::new("testnet.hedera.com:50001")?;
    let target = "0:0:2".parse()?;

    // Get the _cost_ or transaction fee for the query of getting the account balance
    let cost = QueryGetAccountBalance::new(&client, target).cost()?;

    // Wait 1s between queries (limitation of test networks)
    sleep(Duration::from_secs(1));

    // Get the _answer_ for the query of getting the account balance (which is the actual balance)
    let balance = QueryGetAccountBalance::new(&client, target).answer()?;

    println!("cost    = {} tinybars", cost);
    println!("balance = {} tinybars", balance);
    println!("balance = {} hbars", (balance as f64) / 100000000.0);

    Ok(())
}
