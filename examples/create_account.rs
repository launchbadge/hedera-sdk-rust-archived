use failure::{format_err, Error};
use hedera::{crypto::SecretKey, Client, TransactionStatus};
use std::{env, thread::sleep, time::Duration};

fn main() -> Result<(), Error> {
    let secret = SecretKey::generate();
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = "0:0:2".parse()?;

    // We need the secret key for the operator
    let operator_secret = env::var("OPERATOR_SECRET")?.parse()?;

    // Node is the account of the node we are sending the transaction to
    // This should be provided
    let node = "0:0:3".parse()?;

    let client = Client::new("testnet.hedera.com:50001")?;

    // Create our account
    let res = client
        .create_account()
        .key(public)
        .initial_balance(10)
        .operator(operator)
        .node(node)
        .memo("[hedera-sdk-rust][example]")
        .sign(&operator_secret)
        .execute()?;

    println!("created account; transaction = {}", res.id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions

    println!("wait for 2s... ");
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let receipt = client.transaction(res.id).receipt().get()?;
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
