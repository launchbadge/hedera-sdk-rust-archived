use failure::{format_err, Error};
use hedera::{Client, SecretKey, TransactionStatus};
use std::{env, thread::sleep, time::Duration};

fn main() -> Result<(), Error> {
    let (secret, _) = SecretKey::generate("");
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = "0:0:2".parse()?;

    // We need the secret key for the operator
    let operator_secret = env::var("OPERATOR_SECRET")?.parse()?;

    let client = Client::builder("testnet.hedera.com:50001")
        .operator(operator, operator_secret)
        .build()?;

    // Create our account
    let id = client
        .create_account()
        .key(public)
        .initial_balance(10)
        .memo("[hedera-sdk-rust][example] create_account")
        .execute()?;

    println!("created account; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let receipt = client.transaction(id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
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
