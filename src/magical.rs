use crate::NekoHash;
use rand::{Rng, thread_rng};

const MAGIC_CONSTANT: u64 = 0x_CAFE_F00D_DEAD_BEEF;

/// MagicalHash implementation with fixed 16-byte output
pub struct MagicalHash {
    magic_number: u64,
}

impl Default for MagicalHash {
    fn default() -> Self {
        Self::new()
    }
}

impl MagicalHash {
    /// Creates a new MagicalHash with default magic number
    pub fn new() -> Self {
        Self {
            magic_number: MAGIC_CONSTANT,
        }
    }

    /// Creates a new MagicalHash with a custom magic number
    pub fn with_magic(magic: u64) -> Self {
        Self {
            magic_number: magic,
        }
    }
}

impl NekoHash for MagicalHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut state = self.magic_number;
        let mut result = Vec::with_capacity(16);

        // Cast magic spells (mix input)
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u64) << (i * 8);
            }
            
            // Magical transformations
            state ^= value;
            state = state.rotate_left(13);
            state = state.wrapping_mul(0x_1234_5678_9ABC_DEF0);
            state ^= state >> 31;
        }

        // Final enchantment
        let mut rng = thread_rng();
        let sparkle = rng.gen::<u64>();
        state ^= sparkle;
        state = state.wrapping_add(self.magic_number);

        // Create the magical output
        result.extend_from_slice(&state.to_le_bytes());
        result.extend_from_slice(&(state.rotate_right(32)).to_le_bytes());
        
        result
    }

    fn output_size(&self) -> usize {
        16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magical_hash() {
        let hasher = MagicalHash::new();
        let data = b"Hello, Magical World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_magical_hash_custom_magic() {
        let hasher = MagicalHash::with_magic(0x1234_5678_9ABC_DEF0);
        let data = b"Hello, Magical World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_magical_hash_consistency() {
        let hasher = MagicalHash::with_magic(42);
        let data = b"Hello, Magical World!";
        let hash1 = hasher.hash(data);
        let hash2 = hasher.hash(data);
        assert_eq!(hash1, hash2);
    }
}
