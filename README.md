# KornCrypt

KornCrypt is a simple command line encryption and decryption tool using the XChaCha20-Poly1305 algorithm.

## Installation

### From source

You can install KornCrypt from source by cloning the repository and running the following commands:

```
cargo build --release
cargo install --path .
```

## Usage

```
korncrypt 0.1.0
simple encryption tool

USAGE:
    korncrypt <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decrypt         Decrypts a file
    decrypt-text    Decrypts a message
    encrypt         Ecrypts a file
    encrypt-text    Encrypts a message
    generate-key    Generates a new key file The key file is saved to the path specified
    help            Prints this message or the help of the given subcommand(s)
```

### Encrypting files

```
korncrypt encrypt [FLAGS] -k <keyfile> <path_to_file>
```

With the flag `-b` the original file will be removed after encryption.

You can use a password instead of a key file by using the flag `-p` and providing the password as an argument.

#### Example

```
korncrypt encrypt -k key file.txt
```

This will encrypt the file `file.txt` using the key in the file `key` and create a new file `file.txt.kc` with the encrypted content.

```
korncrypt encrypt -p password file.txt
```

This will encrypt the file `file.txt` using the password `password` and create a new file `file.txt.kc` with the encrypted content.

### Decrypting files

```
korncrypt decrypt [FLAGS] -k <keyfile> <path_to_file>
```

With the flag `-b` the original file will be removed after decryption.

You can use a password instead of a key file by using the flag `-p` and providing the password as an argument.

#### Example

```
korncrypt decrypt -k key file.txt.kc
```

This will decrypt the file `file.txt.kc` using the key in the file `key` and create a new file `file.txt` with the decrypted content.

```
korncrypt decrypt -p password file.txt.kc
```

This will decrypt the file `file.txt.kc` using the password `password` and create a new file `file.txt` with the decrypted content.

### Encrypting text

```
korncrypt encrypt-text -k <keyfile> <text>
```

You can use a password instead of a key file by using the flag `-p` and providing the password as an argument.

#### Example

```
korncrypt encrypt-text -k key "Hello, World!"
```

This will encrypt the text `Hello, World!` using the key in the file `key` and print the encrypted text to the console.

```
korncrypt encrypt-text -p password "Hello, World!"
```

This will encrypt the text `Hello, World!` using the password `password` and print the encrypted text to the console.

### Decrypting text

```
korncrypt decrypt-text -k <keyfile> <text>
```

You can use a password instead of a key file by using the flag `-p` and providing the password as an argument.

#### Example

```
korncrypt decrypt-text -k key "encrypted text"
```

This will decrypt the text `encrypted text` using the key in the file `key` and print the decrypted text to the console.

```
korncrypt decrypt-text -p password "encrypted text"
```

This will decrypt the text `encrypted text` using the password `password` and print the decrypted text to the console.

### Generating key file

```
korncrypt generate-key <path>
```

## Be Aware!

This tool is not meant to be used for secure encryption. It is a simple tool to encrypt and decrypt files using a key for educational purposes. The key file should be kept secret and never shared with anyone. Use this tool at your own risk.