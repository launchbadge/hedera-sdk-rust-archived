#![feature(async_await, futures_api, await_macro)]
use chrono::{Duration, Utc};
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, str::from_utf8, thread::sleep, time::Duration as StdDuration};
use tokio::{await, run_async};

async fn main_() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;
    let contents = "Hello World!";

    let mut client = Client::new("testnet.hedera.com:50001")?;
    client.set_operator(operator, || env::var("OPERATOR_SECRET"));

    //
    // Create (empty) File
    //

    let id = await!(client
        .create_file()
        .expires_at(Utc::now() + Duration::minutes(10))
        .key(operator_secret.public())
        .memo("[hedera-sdk-rust][example] create_file : create")
        .sign(&operator_secret)
        .execute_async())?;

    println!("created (empty) file; transaction = {}", id);
    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    //
    // Pull the file receipt (to get the file ID)
    //

    let receipt = await!(client.transaction(id).receipt().get_async())?;
    if receipt.status != Status::Success {
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

    let id = await!(client
        .file(file)
        .append(contents.as_bytes().to_vec())
        .memo("[hedera-sdk-rust][example] create_file : append")
        .sign(&operator_secret)
        .execute_async())?;

    println!("added content to file; transaction = {}", id);
    println!("wait 2s ...");
    sleep(StdDuration::from_secs(2));

    // Pull the receipt; just to be sure it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;
    if receipt.status != Status::Success {
        return Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    //
    // Read the file content
    //

    let file_contents = await!(client.file(file).contents().get_async())?;
    println!("file.contents = {:?}", file_contents);
    println!("file.contents = {:?}", from_utf8(&*file_contents)?);

    //
    // Get more file information
    //

    let file_info = await!(client.file(file).info().get_async())?;
    println!("file.info = {:#?}", file_info);

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
