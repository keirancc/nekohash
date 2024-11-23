# Nekohash

A comprehensive hashing library implementing three distinct hashing algorithms with encryption capabilities. Each algorithm provides different characteristics suitable for various use cases.

## Features

- **KawaiiHash**: A configurable-length hash function using seeded RNG for mixing
- **TsundereHash**: A fixed-size (32-byte) hash with multiple mixing rounds
- **MagicalHash**: A compact (16-byte) hash using state transformation
- **AES-256-CTR Encryption**: Optional encryption layer for hash outputs
- **Utility Functions**: Hash combination, hex conversion, and key management

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nekohash = "0.1.1"
```

or using `cargo`:

```bash
cargo add nekohash
```

## Algorithm Details

### KawaiiHash

KawaiiHash is a variable-length hash function that combines deterministic RNG with bit manipulation:

1. **Initialization**:
   - Configurable output size (default: 32 bytes)
   - Seeded RNG using a 64-bit seed
   - Chunked processing (8 bytes at a time)

2. **Processing Steps**:
   ```rust
   for chunk in data.chunks(8) {
       // Pack bytes into 64-bit value
       let mut value = chunk_to_u64(chunk);
       
       // Apply transformations
       value = value.wrapping_mul(0x1234_5678_9ABC_DEF0);
       value ^= rng.gen::<u64>();
       value = value.rotate_right(17);
       
       // Append to result
       result.extend_from_slice(&value.to_le_bytes());
   }
   ```

Example usage:
```rust
use nekohash::{KawaiiHash, NekoHash};

// Default 32-byte output
let hasher = KawaiiHash::new();
let hash = hasher.hash(b"Example data");

// Custom size (64 bytes)
let hasher = KawaiiHash::with_size(64);
let hash = hasher.hash(b"Example data");

// Custom seed for deterministic output
let hasher = KawaiiHash::with_seed(64, 0xDEADBEEF);
let hash = hasher.hash(b"Example data");
```

### TsundereHash

TsundereHash is a fixed-size hash function using multiple rounds of state transformation:

1. **Characteristics**:
   - 32-byte output size
   - Configurable number of mixing rounds
   - State-based processing with neighboring byte influence

2. **Algorithm Steps**:
   ```rust
   // Initialize 32-byte state
   let mut state = [0u8; 32];
   
   // Initial state mixing
   for (i, &byte) in data.iter().enumerate() {
       state[i % 32] ^= byte;
       state[(i + 7) % 32] = state[(i + 7) % 32].wrapping_add(byte);
       state[i % 32] = if i % 2 == 0 {
           state[i % 32].rotate_left(3)
       } else {
           state[i % 32].rotate_right(2)
       };
   }
   
   // Multiple rounds of transformation
   for _ in 0..rounds {
       for i in 0..32 {
           let prev = state[(i + 31) % 32];
           let next = state[(i + 1) % 32];
           state[i] ^= prev.wrapping_add(next);
           state[i] = state[i].wrapping_mul(0x1B);
       }
   }
   ```

Example usage:
```rust
use nekohash::{TsundereHash, NekoHash};

// Default configuration (3 rounds)
let hasher = TsundereHash::new();
let hash = hasher.hash(b"Example data");

// Custom number of rounds
let hasher = TsundereHash::with_rounds(5);
let hash = hasher.hash(b"Example data");
```

### MagicalHash

MagicalHash is a compact hash function using spell-based transformations:

1. **Characteristics**:
   - 16-byte output size
   - 64-bit internal state
   - Configurable magic constant

2. **Core Transformation**:
   ```rust
   fn cast_spell(value: u64, spell_power: u64) -> u64 {
       let mut result = value;
       result ^= spell_power;
       result = result.rotate_left(7);
       result = result.wrapping_mul(0xDEAD_BEEF_CAFE_BABE);
       result ^= result >> 33;
       result = result.wrapping_mul(0x1234_5678_90AB_CDEF);
       result ^= result >> 29;
       result
   }
   ```

Example usage:
```rust
use nekohash::{MagicalHash, NekoHash};

let hasher = MagicalHash::new();
let hash = hasher.hash(b"Example data");

// Custom magic constant
let hasher = MagicalHash::with_magic(0xCUSTOM_MAGIC);
let hash = hasher.hash(b"Example data");
```

## Encryption Layer

The library provides AES-256-CTR encryption for hash outputs:

1. **Key Management**:
   - 32-byte keys (user-provided or randomly generated)
   - Base64 key encoding for storage
   - Secure IV generation per encryption

2. **Usage Examples**:
```rust
use nekohash::{KawaiiHash, NekoHash, utils};

let hasher = KawaiiHash::new();
let hash = hasher.hash(b"Secret data");

// User-provided key
let key = utils::generate_key();
let key_str = utils::key_to_base64(&key);
let encrypted = hasher.encrypt_hash(&hash, Some(&key))?;
let decrypted = hasher.decrypt_hash(&encrypted, &key)?;

// Random key (unrecoverable)
let encrypted = hasher.encrypt_hash(&hash, None)?;
```

## Hash Combination

The library provides a method to combine multiple hashes:

```rust
use nekohash::{KawaiiHash, TsundereHash, MagicalHash, NekoHash, utils};

let hash1 = KawaiiHash::new().hash(b"Data 1");
let hash2 = TsundereHash::new().hash(b"Data 2");
let hash3 = MagicalHash::new().hash(b"Data 3");

let combined = utils::combine_hashes(&[hash1, hash2, hash3]);
```

The combination algorithm uses a rotating XOR operation to maintain a consistent output size while incorporating all input hashes.

## Cryptographic Utilities

The library includes several cryptographic utility functions for enhanced security and key management:

### Key Management

```rust
// Generate a cryptographically secure key
let key = generate_key();

// Convert key to/from base64 for storage
let key_str = key_to_base64(&key);
let key_back = key_from_base64(&key_str).unwrap();

// Derive a key from password and salt
let salt = generate_salt();
let derived_key = derive_key(b"my_password", &salt);

// Generate time-based keys (useful for temporary tokens)
let seed = b"application_seed";
let temp_key = time_based_key(seed, 30); // 30-second window
```

### Secure Operations

```rust
// Constant-time comparison (timing-attack resistant)
let hash1 = kawaii_hasher.hash(b"data1");
let hash2 = kawaii_hasher.hash(b"data2");
let are_equal = constant_time_compare(&hash1, &hash2);

// Key stretching for password hashing
let stretched = stretch_key(b"password", 10000, 32);

// Combine multiple hashes securely
let combined = combine_hashes(&[hash1, hash2, hash3]);

// Rotate keys for added security
let rotated_key = rotate_key(&key, 12); // rotate 12 bits
```

### Encryption Layer

```rust
// Encrypt with optional key
let encrypted = encrypt_data(b"secret data", None).unwrap();

// Decrypt with key
let decrypted = decrypt_data(&encrypted, &key).unwrap();
```

## Performance Considerations

- KawaiiHash: O(n) complexity, memory usage proportional to output size
- TsundereHash: O(n + r) where r is rounds, constant memory usage
- MagicalHash: O(n) complexity, constant memory usage
- Encryption: Additional O(n) overhead for AES-256-CTR

## Security Notes

These hash functions are designed for general-purpose use and checksums. For cryptographic security, use established cryptographic hash functions. The encryption layer uses standard AES-256-CTR and is suitable for general security needs.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:
- Algorithm improvements
- Performance optimizations
- Additional hash functions
- Enhanced test coverage

## License

This project is licensed under the MIT License - see the LICENSE file for details.
