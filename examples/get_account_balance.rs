use failure::Error;
use hedera::Client;

fn main() -> Result<(), Error> {
    let client = Client::new("testnet.hedera.com:50001");
    let balance = client.get_account_balance("0:0:2".parse()?).answer()?;

    println!("balance = {}", balance);

    Ok(())
}
