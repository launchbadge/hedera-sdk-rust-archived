use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let (secret, _) = SecretKey::generate("");
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = "0:0:2".parse()?;
    let client = Client::builder("testnet.hedera.com:50131")
        .node("0:0:3".parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Create our account
    let id = client
        .create_account()
        .key(public)
        .initial_balance(5_000_000)
        .memo("[hedera-sdk-rust][example] create_account")
        .execute_async()
        .await?;

    println!("created account; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let mut tx = client.transaction(id).receipt();
    let receipt = tx.get_async().await?;

    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    // note: account can be [None] if the receipt wasn't for creating an account
    let account = receipt.account_id.unwrap();
    println!("account = {}", account);

    Ok(())
}
