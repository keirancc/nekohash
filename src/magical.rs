use crate::NekoHash;

/// MagicalHash transforms your input into a magical hash using the power of friendship!
pub struct MagicalHash {
    magic_constant: u64,
}

impl MagicalHash {
    pub fn new() -> Self {
        Self {
            magic_constant: 0xABCD_EF98_7654_3210,
        }
    }

    pub fn with_magic(magic: u64) -> Self {
        Self {
            magic_constant: magic,
        }
    }

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
}

impl NekoHash for MagicalHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(16);
        let mut magic = self.magic_constant;

        // Process input in magical 8-byte chunks
        for chunk in data.chunks(8) {
            let mut value = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u64) << (i * 8);
            }

            magic = Self::cast_spell(value, magic);
        }

        // Final transformation
        magic = Self::cast_spell(magic, data.len() as u64);
        
        // Convert to bytes
        result.extend_from_slice(&magic.to_le_bytes());
        result.extend_from_slice(&(magic >> 32).to_le_bytes());
        
        result
    }

    fn hash_size(&self) -> usize {
        16
    }

    fn algorithm_name(&self) -> &'static str {
        "MagicalHash ⭐️"
    }
}
