use sodiumoxide::base64;
use sodiumoxide::crypto::aead::xchacha20poly1305_ietf;
use sodiumoxide::crypto::pwhash;
use std::fs::File;
use std::io::Read;

// Define a custom error type
#[derive(Debug)]
pub struct EncryptionError {
    message: String,
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EncryptionError {}

pub fn encrypt(
    plaintext: &[u8],
    key: &xchacha20poly1305_ietf::Key,
) -> Result<Vec<u8>, EncryptionError> {
    // Initialize sodiumoxide
    if sodiumoxide::init().is_err() {
        return Err(EncryptionError {
            message: "Failed to initialize sodiumoxide".to_string(),
        });
    }

    let nonce = xchacha20poly1305_ietf::gen_nonce();

    // Encrypt the message
    let ciphertext = xchacha20poly1305_ietf::seal(plaintext, None, &nonce, key);

    let mut combined = nonce.0.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(combined)
}

pub fn decrypt(
    ciphertext: &[u8],
    key: &xchacha20poly1305_ietf::Key,
) -> Result<Vec<u8>, EncryptionError> {
    // Initialize sodiumoxide
    if sodiumoxide::init().is_err() {
        return Err(EncryptionError {
            message: "Failed to initialize sodiumoxide".to_string(),
        });
    }

    // Separate the nonce and ciphertext based on known nonce length
    let nonce_bytes = &ciphertext[..xchacha20poly1305_ietf::NONCEBYTES];
    let ciphertext = &ciphertext[xchacha20poly1305_ietf::NONCEBYTES..];

    // Convert the nonce bytes back to a Nonce type
    let nonce = match xchacha20poly1305_ietf::Nonce::from_slice(nonce_bytes) {
        Some(nonce) => nonce,
        None => {
            return Err(EncryptionError {
                message: "Failed to convert nonce bytes to Nonce type".to_string(),
            })
        }
    };

    // Decrypt the ciphertext
    let decrypted = match xchacha20poly1305_ietf::open(
        ciphertext, None, // Assuming no additional data was used during encryption
        &nonce, key, // Key should be the same as used in encryption
    ) {
        Ok(decrypted) => decrypted,
        Err(_) => {
            return Err(EncryptionError {
                message: "Failed to decrypt ciphertext".to_string(),
            })
        }
    };

    Ok(decrypted)
}

pub fn encrypt_text(
    message: &str,
    key: &xchacha20poly1305_ietf::Key,
) -> Result<String, EncryptionError> {
    // Your message to encrypt
    let message = message.as_bytes();

    let combined = encrypt(message, key)?;

    let encrypted = base64::encode(&combined, base64::Variant::Original);
    Ok(encrypted)
}

pub fn decrypt_text(
    ciphertext: &str,
    key: &xchacha20poly1305_ietf::Key,
) -> Result<String, EncryptionError> {
    let ciphertext = match base64::decode(ciphertext, base64::Variant::Original) {
        Ok(ciphertext) => ciphertext,
        Err(_) => {
            return Err(EncryptionError {
                message: "Failed to decode ciphertext".to_string(),
            })
        }
    };

    let decrypted = decrypt(&ciphertext, key)?;

    Ok(std::str::from_utf8(&decrypted).unwrap().to_string())
}

pub fn gen_key() -> [u8; xchacha20poly1305_ietf::KEYBYTES] {
    // Generate a key
    let key = xchacha20poly1305_ietf::gen_key();

    key.0
}

pub fn read_key(path: &str) -> Result<xchacha20poly1305_ietf::Key, EncryptionError> {
    let mut file = File::open(path).map_err(|_| EncryptionError {
        message: "Failed to open key file".to_string(),
    })?;
    let mut key_bytes = [0u8; xchacha20poly1305_ietf::KEYBYTES]; // Ensure the buffer size matches the key size
    file.read_exact(&mut key_bytes)
        .map_err(|_| EncryptionError {
            message: "Failed to read key from file".to_string(),
        })?;

    xchacha20poly1305_ietf::Key::from_slice(&key_bytes).ok_or_else(|| EncryptionError {
        message: "Failed to convert key bytes to Key type".to_string(),
    })
}

pub fn derive_key_from_password(password: &str) -> xchacha20poly1305_ietf::Key {
    let passwd = password.as_bytes();
    let salt = [
        160, 66, 33, 129, 96, 8, 178, 106, 87, 231, 251, 168, 70, 29, 30, 202, 58, 27, 239, 107,
        20, 136, 115, 242, 183, 237, 31, 138, 153, 84, 132, 99,
    ];
    let salt = pwhash::Salt::from_slice(&salt).unwrap();

    let mut key = [0u8; xchacha20poly1305_ietf::KEYBYTES];

    pwhash::derive_key(
        &mut key,
        passwd,
        &salt,
        pwhash::OPSLIMIT_INTERACTIVE,
        pwhash::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    xchacha20poly1305_ietf::Key::from_slice(&key).unwrap()
}
