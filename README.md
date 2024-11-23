# 🐱 Nekohash

A kawaii hashing library for Rust that keeps you cryptographically secure! ✨

## Features

- 🌸 **KawaiiHash**: A cute and deterministic hashing algorithm (default 32 bytes)
- 💢 **TsundereHash**: A tsundere-themed hash that's reliable... b-baka!
- ⭐ **MagicalHash**: Transforms data using the power of friendship
- 🔐 **Encryption Support**: Optional encryption/decryption of hashes
- 🛠️ **Utility Functions**: Helpful tools for hex conversion, hash combining, and key management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nekohash = "0.1.0"
```

## Usage

### Basic Hashing

```rust
use nekohash::{KawaiiHash, TsundereHash, MagicalHash, NekoHash, utils};

// Create a new KawaiiHash instance (default 32 bytes)
let kawaii = KawaiiHash::new();
let hash = kawaii.hash(b"Hello, Neko!");
println!("KawaiiHash: {}", utils::to_hex(&hash));

// Use TsundereHash with custom rounds
let tsundere = TsundereHash::with_rounds(4);
let hash = tsundere.hash(b"Notice me, senpai!");
println!("TsundereHash: {}", utils::to_hex(&hash));

// Create magical hashes
let magical = MagicalHash::new();
let hash = magical.hash(b"By the power of friendship!");
println!("MagicalHash: {}", utils::to_hex(&hash));
```

### Encryption and Key Management

```rust
use nekohash::{KawaiiHash, NekoHash, utils};

// Create a hash
let hasher = KawaiiHash::new();
let hash = hasher.hash(b"Secret message");

// Generate a random key
let key = utils::generate_key();
let key_str = utils::key_to_base64(&key);
println!("Save this key: {}", key_str);

// Encrypt with key
let encrypted = hasher.encrypt_hash(&hash, Some(&key)).unwrap();
println!("Encrypted: {}", utils::to_hex(&encrypted));

// Decrypt with key
let decrypted = hasher.decrypt_hash(&encrypted, &key).unwrap();
assert_eq!(hash, decrypted);

// Encrypt with random key (unrecoverable)
let encrypted_random = hasher.encrypt_hash(&hash, None).unwrap();
```

### Combining Hashes

```rust
let combined = utils::combine_hashes(&[
    kawaii.hash(b"uwu"),
    tsundere.hash(b"owo"),
    magical.hash(b"nya~"),
]);
println!("Combined Hash: {}", utils::to_hex(&combined));
```

## Hash Algorithms

### KawaiiHash
A deterministic hashing algorithm that uses kawaii magic numbers and bit manipulation to create unique hashes. Configurable output size (default 32 bytes) and seed value.

### TsundereHash
A reliable hashing algorithm with a tsundere personality. Uses multiple rounds of mixing with special constants. Fixed 32-byte output size.

### MagicalHash
Transforms input data using magical girl powers and friendship. Uses special spell-casting functions for mixing. Fixed 16-byte output size.

## Encryption Details

The library uses AES-256-CTR for encryption with the following features:
- Optional key provision (32 bytes)
- Random key generation if no key is provided
- Base64 key storage format
- Secure IV generation per encryption
- Built-in encryption/decryption methods for all hash types

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

While these hashing algorithms are fun and educational, they are not cryptographically secure. For serious cryptographic needs, please use established cryptographic hash functions. The encryption functionality, however, uses standard cryptographic primitives (AES-256-CTR) and is suitable for general use.
