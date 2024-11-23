pub mod kawaii;
pub mod tsundere;
pub mod magical;
pub mod utils;

pub use kawaii::KawaiiHash;
pub use tsundere::TsundereHash;
pub use magical::MagicalHash;

/// The main trait that all Nekohash algorithms implement
pub trait NekoHash {
    /// Computes the hash of the input data
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    
    /// Returns the size of the hash in bytes
    fn hash_size(&self) -> usize;
    
    /// Returns a friendly name for the hash algorithm
    fn algorithm_name(&self) -> &'static str;

    /// Encrypts the hash with an optional key
    /// If no key is provided, a random key will be generated and the hash will be unrecoverable
    fn encrypt_hash(&self, hash: &[u8], key: Option<&[u8]>) -> Result<Vec<u8>, String> {
        utils::encrypt_data(hash, key)
    }

    /// Attempts to decrypt an encrypted hash with the given key
    fn decrypt_hash(&self, encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
        utils::decrypt_data(encrypted, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_kawaii_hash_encryption() {
        let hasher = KawaiiHash::new();
        let data = b"Hello, World!";
        let hash = hasher.hash(data);
        
        // Test with provided key
        let key = [42u8; 32];
        let encrypted = hasher.encrypt_hash(&hash, Some(&key)).unwrap();
        let decrypted = hasher.decrypt_hash(&encrypted, &key).unwrap();
        assert_eq!(hash, decrypted);

        // Test with random key (should fail with different key)
        let encrypted_random = hasher.encrypt_hash(&hash, None).unwrap();
        let wrong_key = [7u8; 32];
        let result = hasher.decrypt_hash(&encrypted_random, &wrong_key);
        assert!(result.is_err() || result.unwrap() != hash);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
