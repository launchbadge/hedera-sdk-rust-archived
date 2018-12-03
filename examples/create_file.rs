use chrono::{Duration, Utc};
use failure::{format_err, Error};
use hedera::{crypto::SecretKey, Client, TransactionStatus};
use std::{env, thread::sleep, time::Duration as StdDuration};

fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;
    let node = "0:0:3".parse()?;
    let contents = "Hello World!";

    let client = Client::new("testnet.hedera.com:50001")?;

    //
    // Create (empty) File
    //

    let res = client
        .create_file()
        .expires_at(Utc::now() + Duration::minutes(10))
        .key(operator_secret.public())
        .operator(operator)
        .node(node)
        .memo("[hedera-sdk-rust][example] create_file : create")
        .sign(&operator_secret)
        .sign(&operator_secret)
        .execute()?;

    println!("created (empty) file; transaction = {}", res.id);
    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    //
    // Pull the file receipt (to get the file ID)
    //

    let receipt = client.transaction(res.id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
        return Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    let file = *receipt.file_id.unwrap();
    println!("file = {}", file);

    //
    // Append some content to the file
    //

    let res = client
        .file(file)
        .append(contents.as_bytes().to_vec())
        .operator(operator)
        .node(node)
        .memo("[hedera-sdk-rust][example] create_file : append")
        .sign(&operator_secret)
        .sign(&operator_secret)
        .execute()?;

    println!("added content to file; transaction = {}", res.id);
    println!("wait 10s ...");
    sleep(StdDuration::from_secs(10));

    // Pull the receipt; just to be sure it was successful
    let receipt = client.transaction(res.id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
        return Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    //
    // TODO: Read the file content
    //

    Ok(())
}
