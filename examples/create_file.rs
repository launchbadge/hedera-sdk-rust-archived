use chrono::{Duration, Utc};
use failure::{format_err, Error};
use hedera::{Client, SecretKey, TransactionStatus};
use std::str::from_utf8;
use std::{env, thread::sleep, time::Duration as StdDuration};

// todo: default file owner

fn main() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;
    let contents = "Hello World!";

    let client = Client::builder("testnet.hedera.com:50001")
        .operator(operator, operator_secret.clone())
        .build()?;

    //
    // Create (empty) File
    //

    let id = client
        .create_file()
        .expires_at(Utc::now() + Duration::minutes(10))
        .key(operator_secret.public())
        .memo("[hedera-sdk-rust][example] create_file : create")
        .sign(&operator_secret)
        .execute()?;

    println!("created (empty) file; transaction = {}", id);
    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    //
    // Pull the file receipt (to get the file ID)
    //

    let receipt = client.transaction(id).receipt().get()?;
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

    let id = client
        .file(file)
        .append(contents.as_bytes().to_vec())
        .memo("[hedera-sdk-rust][example] create_file : append")
        .sign(&operator_secret)
        .execute()?;

    println!("added content to file; transaction = {}", id);
    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    // Pull the receipt; just to be sure it was successful
    let receipt = client.transaction(id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
        return Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    //
    // Read the file content
    //

    let file_contents_cost = client.file(file).contents().cost()?;
    println!("cost:file.contents = {} tinybars", file_contents_cost);

    let file_contents = client.file(file).contents().get()?;
    println!("file.contents = {:?}", file_contents);
    println!("file.contents = {:?}", from_utf8(&*file_contents)?);

    //
    // Get more file information
    //

    let file_info_cost = client.file(file).info().cost()?;
    println!("cost:file.info = {} tinybars", file_info_cost);

    let file_info = client.file(file).info().get()?;
    println!("file.info = {:#?}", file_info);

    Ok(())
}
