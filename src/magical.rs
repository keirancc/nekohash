use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::NekoHash;

const MAGIC_CONSTANT: u32 = 0x19_95_08_16;

/// MagicalHash implementation with fixed 16-byte output
pub struct MagicalHash {
    magic: u32,
    rng: StdRng,
}

impl Default for MagicalHash {
    fn default() -> Self {
        Self::new()
    }
}

impl MagicalHash {
    /// Creates a new MagicalHash with default magic number
    pub fn new() -> Self {
        Self::with_magic(MAGIC_CONSTANT)
    }

    /// Creates a new MagicalHash with a custom magic number
    pub fn with_magic(magic: u32) -> Self {
        Self {
            magic,
            rng: StdRng::seed_from_u64(magic as u64),
        }
    }
}

impl NekoHash for MagicalHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = vec![0u8; 16];
        let mut rng = StdRng::seed_from_u64(self.magic as u64);

        // Initialize result with magic number
        for i in 0..4 {
            let magic_bytes = self.magic.to_le_bytes();
            result[i*4..(i+1)*4].copy_from_slice(&magic_bytes);
        }

        // Mix in input data
        for (i, &byte) in data.iter().enumerate() {
            let idx = i % 16;
            result[idx] ^= byte;
            result[idx] = result[idx].rotate_left(3);
            
            let random = rng.gen::<u8>();
            result[idx] = result[idx].wrapping_add(random);
        }

        // Apply magical transformations
        for i in 0..4 {
            let mut value = u32::from_le_bytes([
                result[i*4],
                result[i*4 + 1],
                result[i*4 + 2],
                result[i*4 + 3],
            ]);

            value = value.wrapping_mul(self.magic);
            value = value.rotate_left(7);
            value ^= self.magic;

            let bytes = value.to_le_bytes();
            result[i*4..(i+1)*4].copy_from_slice(&bytes);
        }

        // Final mixing
        for i in 0..16 {
            let random = rng.gen::<u8>();
            result[i] = result[i].wrapping_add(random);
            result[i] = result[i].rotate_left(3);
        }

        result
    }

    fn output_size(&self) -> usize {
        16
    }

    fn reset(&mut self) {
        self.rng = StdRng::seed_from_u64(self.magic as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magical_hash() {
        let hasher = MagicalHash::new();
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_magical_hash_custom_magic() {
        let hasher = MagicalHash::with_magic(0xCAFEBABE);
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_magical_hash_consistency() {
        let hasher = MagicalHash::new();
        let input = b"Hello, World!";
        let hash1 = hasher.hash(input);
        let hash2 = hasher.hash(input);
        assert_eq!(hash1, hash2);
    }
}
