# 🐱 Nekohash

An anime-themed hashing library for Rust that brings kawaii energy to your cryptographic needs! ✨

## Features

- 🌸 **KawaiiHash**: A cute and deterministic hashing algorithm
- 💢 **TsundereHash**: A tsundere-themed hash that's reliable... b-baka!
- ⭐ **MagicalHash**: Transforms data using the power of friendship
- 🛠️ **Utility Functions**: Helpful tools for hex conversion and hash combining

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nekohash = "0.1.0"
```

## Usage

```rust
use nekohash::{KawaiiHash, TsundereHash, MagicalHash, NekoHash, utils};

// Create a new KawaiiHash instance
let kawaii = KawaiiHash::new(32);
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

// Combine multiple hashes
let combined = utils::combine_hashes(&[
    kawaii.hash(b"uwu"),
    tsundere.hash(b"owo"),
    magical.hash(b"nya~"),
]);
println!("Combined Hash: {}", utils::to_hex(&combined));
```

## Hash Algorithms

### KawaiiHash
A deterministic hashing algorithm that uses kawaii magic numbers and bit manipulation to create unique hashes. Configurable output size and seed value.

### TsundereHash
A reliable hashing algorithm with a tsundere personality. Uses multiple rounds of mixing with special constants. Fixed 32-byte output size.

### MagicalHash
Transforms input data using magical girl powers and friendship. Uses special spell-casting functions for mixing. Fixed 16-byte output size.

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

While these hashing algorithms are fun and educational, they are not cryptographically secure. For serious cryptographic needs, please use established cryptographic hash functions.
