use hedera::crypto::SecretKey;

fn main() {
    let secret = SecretKey::generate();
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);
}
