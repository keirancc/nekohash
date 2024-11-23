use rand::{Rng, thread_rng};
use crate::NekoHash;

/// KawaiiHash implementation with configurable output size
pub struct KawaiiHash {
    size: usize,
    seed: u64,
}

impl Default for KawaiiHash {
    fn default() -> Self {
        Self::new()
    }
}

impl KawaiiHash {
    /// Creates a new KawaiiHash with default size (32 bytes)
    pub fn new() -> Self {
        Self::with_size(32)
    }

    /// Creates a new KawaiiHash with specified output size
    pub fn with_size(size: usize) -> Self {
        Self {
            size,
            seed: thread_rng().gen(),
        }
    }

    /// Creates a new KawaiiHash with specified size and seed
    pub fn with_seed_and_size(seed: u64, size: usize) -> Self {
        Self { size, seed }
    }
}

impl NekoHash for KawaiiHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.size);
        let mut state = self.seed;

        // Initial mixing
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u64) << (i * 8);
            }
            state = state.wrapping_add(value);
            state = state.rotate_left(13);
            state ^= value;
        }

        // Generate output
        let mut rng = thread_rng();
        while result.len() < self.size {
            state = state.wrapping_mul(0x6c50_8bbb_9c09_c9df);
            state ^= state >> 32;
            state = state.wrapping_add(rng.gen::<u64>());
            result.extend_from_slice(&state.to_le_bytes());
        }

        result.truncate(self.size);
        result
    }

    fn output_size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kawaii_hash() {
        let hasher = KawaiiHash::new();
        let data = b"Hello, World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_kawaii_hash_custom_size() {
        let hasher = KawaiiHash::with_size(16);
        let data = b"Hello, World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_kawaii_hash_deterministic() {
        let hasher = KawaiiHash::with_seed_and_size(42, 32);
        let data = b"Hello, World!";
        let hash1 = hasher.hash(data);
        let hash2 = hasher.hash(data);
        assert_eq!(hash1, hash2);
    }
}
