use chrono::{Duration, Utc};
use failure::{format_err, Error};
use hedera::{crypto::SecretKey, Client, TransactionStatus};
use std::{env, thread::sleep, time::Duration as StdDuration};

fn main() -> Result<(), Error> {
    let operator = env::var("OPERATOR")?.parse()?;
    let operator_secret: SecretKey = env::var("OPERATOR_SECRET")?.parse()?;
    let node = "0:0:3".parse()?;

    let client = Client::new("testnet.hedera.com:50002")?;

    let res = client
        .create_file()
        .expires_at(Utc::now() + Duration::minutes(10))
        .key(operator_secret.public())
        .operator(operator)
        .node(node)
        .memo("[hedera-sdk-rust][example] create_file")
        .sign(operator_secret.clone())
        .sign(operator_secret)
        .execute()?;

    println!("created (empty) file; transaction = {}", res.id);

    sleep(StdDuration::from_secs(20));

    let receipt = client.transaction(res.id).receipt().get()?;
    if receipt.status != TransactionStatus::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    let file = receipt.file_id.unwrap();
    println!("file = {}", file);

    Ok(())
}
