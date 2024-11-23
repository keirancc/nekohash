use crate::NekoHash;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// KawaiiHash is a cute and deterministic hashing algorithm that uses
/// a combination of bit manipulation and kawaii magic to create hashes.
pub struct KawaiiHash {
    seed: u64,
    size: usize,
}

impl Default for KawaiiHash {
    fn default() -> Self {
        Self::new()
    }
}

impl KawaiiHash {
    /// Creates a new KawaiiHash with the default size of 32 bytes
    pub fn new() -> Self {
        Self::with_size(32)
    }

    /// Creates a new KawaiiHash with a specific size
    pub fn with_size(size: usize) -> Self {
        Self {
            seed: 0xCAFEBABE,
            size,
        }
    }

    /// Creates a new KawaiiHash with a specific size and seed
    pub fn with_seed(size: usize, seed: u64) -> Self {
        Self { seed, size }
    }
}

impl NekoHash for KawaiiHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut result = Vec::with_capacity(self.size);
        
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u64) << (i * 8);
            }
            
            // Mix with kawaii magic numbers
            value = value.wrapping_mul(0x1234_5678_9ABC_DEF0);
            value ^= rng.gen::<u64>();
            value = value.rotate_right(17);
            
            // Add to result
            result.extend_from_slice(&value.to_le_bytes()[..std::cmp::min(8, self.size - result.len())]);
            if result.len() >= self.size {
                break;
            }
        }
        
        result
    }

    fn hash_size(&self) -> usize {
        self.size
    }

    fn algorithm_name(&self) -> &'static str {
        "KawaiiHash ✨"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_size() {
        let hasher = KawaiiHash::new();
        assert_eq!(hasher.hash_size(), 32);
    }

    #[test]
    fn test_custom_size() {
        let hasher = KawaiiHash::with_size(64);
        assert_eq!(hasher.hash_size(), 64);
    }

    #[test]
    fn test_hash_consistency() {
        let hasher = KawaiiHash::new();
        let data = b"Hello, Kawaii World!";
        let hash1 = hasher.hash(data);
        let hash2 = hasher.hash(data);
        assert_eq!(hash1, hash2);
    }
}
