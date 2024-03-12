mod encryption;

fn main() {
    let key = encryption::read_key("secret.key");

    let message = "Hello, world!";

    let encrypted = encryption::encrypt(message, &key);

    println!("Encrypted: {}", encrypted);

    let decrypted = encryption::decrypt(&encrypted, &key);

    println!("Decrypted: {}", decrypted);
}
