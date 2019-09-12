use hedera::SecretKey;

fn main() {
    let (secret, mnemonic) = SecretKey::generate_mnemonic();
    let public = secret.public();

    println!("secret   = {}", secret);
    println!("mnemonic = {}", mnemonic);
    println!("public   = {}", public);
}
