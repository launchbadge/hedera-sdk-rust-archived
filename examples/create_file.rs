use chrono::{Duration, Utc};
use failure::{format_err, Error};
use hedera::{Client, SecretKey, TransactionStatus};
use std::str::from_utf8;
use std::{env, thread::sleep, time::Duration as StdDuration};

// todo: default query payments (within reason)
// todo: default operator
// todo: default node
// todo: default file owner

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

    let id = client
        .create_file()
        .expires_at(Utc::now() + Duration::minutes(10))
        .key(operator_secret.public())
        .operator(operator, operator_secret.clone())
        .node(node)
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
        .operator(operator, operator_secret.clone())
        .node(node)
        .memo("[hedera-sdk-rust][example] create_file : append")
        .sign(&operator_secret)
        .execute()?;

    println!("added content to file; transaction = {}", id);
    println!("wait 5s ...");
    sleep(StdDuration::from_secs(5));

    // Pull the receipt; just to be sure it was successful
    let receipt = client.transaction(id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
        return Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    println!("wait 10s ...");
    sleep(StdDuration::from_secs(10));

    //
    // Read the file content
    //

    let file_contents_cost = client.file(file).contents().cost()?;

    println!("cost:file.contents = {} tinybars", file_contents_cost);

    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    let file_contents = client
        .file(file)
        .contents()
        .payment(
            client
                .transfer_crypto()
                .operator(operator, operator_secret.clone())
                .node(node)
                .transfer(node, file_contents_cost as i64)
                .transfer(operator, -(file_contents_cost as i64))
                .sign(&operator_secret),
        )?
        .get()?;

    println!("file.contents = {:?}", file_contents);
    println!("file.contents = {:?}", from_utf8(&*file_contents)?);

    //
    // Get more file information
    //

    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    let file_info_cost = client.file(file).info().cost()?;

    println!("cost:file.info = {} tinybars", file_info_cost);

    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    let file_info = client
        .file(file)
        .info()
        .payment(
            client
                .transfer_crypto()
                .operator(operator, operator_secret.clone())
                .node(node)
                .transfer(node, file_info_cost as i64)
                .transfer(operator, -(file_info_cost as i64))
                .sign(&operator_secret),
        )?
        .get()?;

    println!("file.info = {:#?}", file_info);

    Ok(())
}
