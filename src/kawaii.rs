use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::{NekoHash};

/// KawaiiHash implementation with configurable output size
pub struct KawaiiHash {
    size: usize,
    seed: u64,
    rng: StdRng,
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
        let seed = 0xDEADBEEF;
        Self {
            size,
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Creates a new KawaiiHash with specified seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            size: 32,
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Creates a new KawaiiHash with specified size and seed
    pub fn with_size_and_seed(size: usize, seed: u64) -> Self {
        Self {
            size,
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl NekoHash for KawaiiHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = vec![0u8; self.size];
        let mut rng = StdRng::seed_from_u64(self.seed);

        // Initialize result with input data
        for (i, &byte) in data.iter().enumerate() {
            result[i % self.size] ^= byte;
        }

        // Apply kawaii transformations
        for i in 0..self.size {
            let random = rng.gen::<u8>();
            result[i] = result[i].wrapping_add(random);
            result[i] = result[i].rotate_left(3);
            
            if i > 0 {
                result[i] ^= result[i - 1];
            }
        }

        // Final mixing
        for i in (0..self.size).rev() {
            let random = rng.gen::<u8>();
            result[i] = result[i].wrapping_mul(0xB5);
            result[i] ^= random;
            
            if i < self.size - 1 {
                result[i] ^= result[i + 1];
            }
        }

        result
    }

    fn output_size(&self) -> usize {
        self.size
    }

    fn reset(&mut self) {
        self.rng = StdRng::seed_from_u64(self.seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kawaii_hash() {
        let hasher = KawaiiHash::new();
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_kawaii_hash_custom_size() {
        let hasher = KawaiiHash::with_size(16);
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_kawaii_hash_deterministic() {
        let hasher1 = KawaiiHash::with_seed(12345);
        let hasher2 = KawaiiHash::with_seed(12345);
        let input = b"Hello, World!";
        
        let hash1 = hasher1.hash(input);
        let hash2 = hasher2.hash(input);
        
        assert_eq!(hash1, hash2);
    }
}
