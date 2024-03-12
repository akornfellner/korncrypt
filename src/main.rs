use std::{fmt::Error, fs::File, io::Write};

mod encryption;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "simple encryption tool")]
enum Opt {
    /// Encrypts a message
    Encrypt {
        plaintext: String,
        #[structopt(short, long)]
        key: String,
    },
    /// Decrypts a message
    Decrypt {
        ciphertext: String,
        #[structopt(short, long)]
        key: String,
    },
    /// Generates a new key file
    /// The key file is saved to the path specified
    GenerateKey { path: String },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Encrypt { plaintext, key } => {
            let key = match encryption::read_key(&key) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("Failed to read key");
                    return;
                }
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
        Opt::Decrypt { ciphertext, key } => {
            let key = match encryption::read_key(&key) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("Failed to read key");
                    return;
                }
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
