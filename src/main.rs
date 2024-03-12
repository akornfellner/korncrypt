use std::{
    fmt::Error,
    fs::File,
    io::{self, Read, Write},
};

mod encryption;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "simple encryption tool")]
enum Opt {
    /// Ecrypts a file
    Encrypt {
        /// the key file
        #[structopt(short, long)]
        key: Option<String>,
        /// the password (will not be used if a key file is provided)
        #[structopt(short, long)]
        password: Option<String>,
        /// remove the original file
        #[structopt(short, long)]
        remove: bool,
        file: String,
    },
    /// Decrypts a file
    Decrypt {
        /// the key file
        #[structopt(short, long)]
        key: Option<String>,
        /// the password (will not be used if a key file is provided)
        #[structopt(short, long)]
        password: Option<String>,
        /// remove the original file
        #[structopt(short, long)]
        remove: bool,
        file: String,
    },
    /// Encrypts a message
    EncryptText {
        /// the key file
        #[structopt(short, long)]
        key: Option<String>,
        /// the password (will not be used if a key file is provided)
        #[structopt(short, long)]
        password: Option<String>,
        plaintext: String,
    },
    /// Decrypts a message
    DecryptText {
        /// the key file
        #[structopt(short, long)]
        key: Option<String>,
        /// the password (will not be used if a key file is provided)
        #[structopt(short, long)]
        password: Option<String>,
        ciphertext: String,
    },
    /// Generates a new key file
    /// The key file is saved to the path specified
    GenerateKey { path: String },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Encrypt {
            key,
            password,
            file,
            remove,
        } => {
            let key = match key {
                Some(key) => encryption::read_key(&key).unwrap(),
                None => match password {
                    Some(password) => encryption::derive_key_from_password(&password),
                    None => {
                        eprintln!("No key or password provided");
                        return;
                    }
                },
            };

            let plaintext = match read_file(&file) {
                Ok(plaintext) => plaintext,
                Err(_) => {
                    eprintln!("Failed to read file");
                    return;
                }
            };

            let ciphertext = match encryption::encrypt_file(&plaintext, &key) {
                Ok(ciphertext) => ciphertext,
                Err(_) => {
                    eprintln!("Failed to encrypt file");
                    return;
                }
            };

            let path = file.clone() + ".kc";

            if write_file(&path, &ciphertext).is_err() {
                eprintln!("Failed to write encrypted file");
                return;
            }

            if remove && std::fs::remove_file(&file).is_err() {
                eprintln!("Failed to remove original file");
            }
        }
        Opt::Decrypt {
            key,
            password,
            file,
            remove,
        } => {
            let key = match key {
                Some(key) => encryption::read_key(&key).unwrap(),
                None => match password {
                    Some(password) => encryption::derive_key_from_password(&password),
                    None => {
                        eprintln!("No key or password provided");
                        return;
                    }
                },
            };

            let ciphertext = match read_file(&file) {
                Ok(ciphertext) => ciphertext,
                Err(_) => {
                    eprintln!("Failed to read file");
                    return;
                }
            };

            let plaintext = match encryption::decrypt_file(&ciphertext, &key) {
                Ok(plaintext) => plaintext,
                Err(_) => {
                    eprintln!("Failed to decrypt file");
                    return;
                }
            };

            let path = file.replace(".kc", "");

            if write_file(&path, &plaintext).is_err() {
                eprintln!("Failed to write decrypted file");
                return;
            }

            if remove && std::fs::remove_file(&file).is_err() {
                eprintln!("Failed to remove original file");
            }
        }
        Opt::EncryptText {
            plaintext,
            key,
            password,
        } => {
            let key = match key {
                Some(key) => encryption::read_key(&key).unwrap(),
                None => match password {
                    Some(password) => encryption::derive_key_from_password(&password),
                    None => {
                        eprintln!("No key or password provided");
                        return;
                    }
                },
            };

            let ciphertext = match encryption::encrypt(&plaintext, &key) {
                Ok(ciphertext) => ciphertext,
                Err(_) => {
                    eprintln!("Failed to encrypt message");
                    return;
                }
            };

            println!("{}", ciphertext);
        }
        Opt::DecryptText {
            ciphertext,
            key,
            password,
        } => {
            let key = match key {
                Some(key) => encryption::read_key(&key).unwrap(),
                None => match password {
                    Some(password) => encryption::derive_key_from_password(&password),
                    None => {
                        eprintln!("No key or password provided");
                        return;
                    }
                },
            };

            let plaintext = match encryption::decrypt(&ciphertext, &key) {
                Ok(plaintext) => plaintext,
                Err(_) => {
                    eprintln!("Failed to decrypt message");
                    return;
                }
            };

            println!("{}", plaintext);
        }
        Opt::GenerateKey { path } => {
            let key = encryption::gen_key();
            if save_key(&key, &path).is_err() {
                eprintln!("Failed to save key");
                return;
            }
            println!("Key generated successfully")
        }
    }
}

fn save_key(key: &[u8], path: &str) -> Result<(), Error> {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => return Err(Error),
    };

    if file.write_all(key).is_err() {
        return Err(Error);
    }

    Ok(())
}

fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_file(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)
}
