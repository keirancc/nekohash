use std::error::Error;
use std::fmt;

pub mod kawaii;
pub mod magical;
pub mod tsundere;
pub mod utils;

/// Custom error type for the Nekohash library
#[derive(Debug)]
pub enum NekoError {
    /// Error during hash computation
    HashError(String),
    /// Error during encryption/decryption
    CryptoError(String),
    /// Error during key operations
    KeyError(String),
    /// Error during encoding/decoding
    EncodingError(String),
    /// Invalid input parameters
    InvalidInput(String),
    /// IO operation error
    IoError(std::io::Error),
}

impl fmt::Display for NekoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NekoError::HashError(msg) => write!(f, "Hash error: {}", msg),
            NekoError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            NekoError::KeyError(msg) => write!(f, "Key error: {}", msg),
            NekoError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            NekoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            NekoError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for NekoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NekoError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for NekoError {
    fn from(err: std::io::Error) -> Self {
        NekoError::IoError(err)
    }
}

/// Result type for Nekohash operations
pub type NekoResult<T> = Result<T, NekoError>;

/// Trait for hash implementations
pub trait NekoHash {
    /// Hash the input data
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    
    /// Hash the input data with encryption
    fn hash_encrypted(&self, data: &[u8], key: Option<&[u8]>) -> NekoResult<Vec<u8>> {
        let hash = self.hash(data);
        utils::encrypt_data(&hash, key)
    }
    
    /// Get the output size of the hash in bytes
    fn output_size(&self) -> usize;
    
    /// Reset the hash state if applicable
    fn reset(&mut self) {
        // Default implementation does nothing
    }
}

pub use kawaii::KawaiiHash;
pub use magical::MagicalHash;
pub use tsundere::TsundereHash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_encryption() {
        let hasher = KawaiiHash::new();
        let data = b"Hello, World!";
        let hash = hasher.hash(data);
        
        // Test with provided key
        let key = [42u8; 32];
        let encrypted = hasher.hash_encrypted(data, Some(&key)).unwrap();
        let decrypted = utils::decrypt_data(&encrypted, &key).unwrap();
        assert_eq!(hash, decrypted);

        // Test with random key (should fail with different key)
        let encrypted_random = hasher.hash_encrypted(data, None).unwrap();
        let wrong_key = [7u8; 32];
        let result = utils::decrypt_data(&encrypted_random, &wrong_key);
        assert!(result.is_err() || result.unwrap() != hash);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
