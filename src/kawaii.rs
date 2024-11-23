use crate::NekoHash;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// KawaiiHash is a cute and deterministic hashing algorithm that uses
/// a combination of bit manipulation and kawaii magic to create hashes.
pub struct KawaiiHash {
    seed: u64,
    size: usize,
}

impl KawaiiHash {
    pub fn new(size: usize) -> Self {
        Self {
            seed: 0xCAFEBABE,
            size,
        }
    }

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
