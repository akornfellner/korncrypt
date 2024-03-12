use sodiumoxide::base64;
use sodiumoxide::crypto::aead::xchacha20poly1305_ietf;
use std::fs::File;
use std::io::{Read, Write};

pub fn encrypt(message: &str, key: &xchacha20poly1305_ietf::Key) -> String {
    // Initialize sodiumoxide
    sodiumoxide::init().unwrap();

    let nonce = xchacha20poly1305_ietf::gen_nonce();

    // Your message to encrypt
    let message = message.as_bytes();

    // Encrypt the message
    let ciphertext = xchacha20poly1305_ietf::seal(message, None, &nonce, key);

    let mut combined = nonce.0.to_vec();
    combined.extend_from_slice(&ciphertext);

    let encrypted = base64::encode(&combined, base64::Variant::Original);
    encrypted
}

pub fn decrypt(ciphertext: &str, key: &xchacha20poly1305_ietf::Key) -> String {
    // Initialize sodiumoxide
    sodiumoxide::init().unwrap();

    let ciphertext = base64::decode(ciphertext, base64::Variant::Original).unwrap();

    // Separate the nonce and ciphertext based on known nonce length
    let nonce_bytes = &ciphertext[..xchacha20poly1305_ietf::NONCEBYTES];
    let ciphertext = &ciphertext[xchacha20poly1305_ietf::NONCEBYTES..];

    // Convert the nonce bytes back to a Nonce type
    let nonce =
        xchacha20poly1305_ietf::Nonce::from_slice(nonce_bytes).expect("Failed to create nonce");

    // Decrypt the ciphertext
    let decrypted = xchacha20poly1305_ietf::open(
        ciphertext, None, // Assuming no additional data was used during encryption
        &nonce, key, // Key should be the same as used in encryption
    )
    .expect("Decryption failed");

    std::str::from_utf8(&decrypted).unwrap().to_string()
}

pub fn gen_key(path: &str) -> xchacha20poly1305_ietf::Key {
    // Generate a key
    let key = xchacha20poly1305_ietf::gen_key();

    let key_bytes = key.0;

    // Create a .key file and write the key bytes to it
    let mut file = File::create(path).expect("Failed to create key file");
    file.write_all(&key_bytes)
        .expect("Failed to write key to file");
    println!("Key has been written to secret.key");

    key
}

pub fn read_key(path: &str) -> xchacha20poly1305_ietf::Key {
    let mut file = File::open(path).expect("Failed to open key file");
    let mut key_bytes = [0u8; xchacha20poly1305_ietf::KEYBYTES]; // Ensure the buffer size matches the key size
    file.read_exact(&mut key_bytes)
        .expect("Failed to read key from file");

    xchacha20poly1305_ietf::Key::from_slice(&key_bytes).unwrap()
}
