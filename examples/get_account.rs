use failure::Error;
use hedera::Client;
use std::env;

// This example gets balance and information for an existing account

// to invoke from unix/macOs terminal
// export OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// export NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// export NODE_ACCOUNT=node's account (e.g. 0.0.3)
// export OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// export ACCOUNT_ID=the account you wish to query (e.g. 0.0.2)
// then from the hedera-sdk-rust root run:
// cargo run --example get_account

// to invoke from windows command line
// set OPERATOR=The account ID executing the transaction (e.g. 0.0.2)
// set NODE_PORT=node:port you're sending the transaction to (e.g. testnet.hedera.com:50003)
// set NODE_ACCOUNT=node's account (e.g. 0.0.3)
// set OPERATOR_SECRET=your private key (e.g. 302e020100300506032b657004220420aaeeb4f94573f3d13b4f0965d4e59d1cf30695d9d9788d25539f322bdf3a5edd)
// set ACCOUNT_ID=the account you wish to query (e.g. 0.0.2)
// then from the hedera-sdk-rust root run:
// cargo run --example get_account

fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let operator = env::var("OPERATOR")?.parse()?;
    let query_account = env::var("ACCOUNT_ID")?.parse()?;
    let node_port : String = env::var("NODE_PORT")?;
    let client = Client::builder(&node_port)
        .node(env::var("NODE_ACCOUNT")?.parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar

    let balance = client.account(query_account).balance().get()?;
    println!("balance = {} tinybars", balance);

    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    let info = client.account(query_account).info().get()?;
    println!("info = {:#?}", info);

    Ok(())
}
