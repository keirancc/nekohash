use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::NekoHash;

/// TsundereHash implementation with fixed 32-byte output
pub struct TsundereHash {
    rounds: usize,
    state: Vec<u8>,
    rng: StdRng,
}

impl Default for TsundereHash {
    fn default() -> Self {
        Self::new()
    }
}

impl TsundereHash {
    /// Creates a new TsundereHash with default settings
    pub fn new() -> Self {
        Self::with_rounds(8)
    }

    /// Creates a new TsundereHash with specified number of rounds
    pub fn with_rounds(rounds: usize) -> Self {
        let seed = 0x544e554e44455245; // 0xTSUNDERE
        Self {
            rounds,
            state: vec![0; 32],
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl NekoHash for TsundereHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = self.state.clone();
        let mut rng = self.rng.clone();

        // Initialize state with input
        for (i, &byte) in data.iter().enumerate() {
            result[i % 32] ^= byte;
        }

        // Apply tsundere transformations
        for _ in 0..self.rounds {
            // First pass - mix with random values
            for i in 0..32 {
                let random = rng.gen::<u8>();
                result[i] = result[i].wrapping_add(random);
                result[i] = result[i].rotate_left(3);
            }

            // Second pass - mix with previous values
            for i in 1..32 {
                result[i] ^= result[i - 1];
            }

            // Third pass - mix with future values
            for i in (0..31).rev() {
                result[i] ^= result[i + 1];
            }

            // Fourth pass - apply tsundere magic
            for i in 0..32 {
                let random = rng.gen::<u8>();
                result[i] = result[i].wrapping_mul(0xB5);
                result[i] ^= random;
            }
        }

        result
    }

    fn output_size(&self) -> usize {
        32
    }

    fn reset(&mut self) {
        self.state = vec![0; 32];
        self.rng = StdRng::seed_from_u64(0x544e554e44455245); // 0xTSUNDERE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsundere_hash() {
        let hasher = TsundereHash::new();
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_tsundere_hash_custom_rounds() {
        let hasher = TsundereHash::with_rounds(16);
        let input = b"Hello, World!";
        let hash = hasher.hash(input);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_tsundere_hash_reset() {
        let mut hasher = TsundereHash::new();
        let input = b"Hello, World!";
        
        let hash1 = hasher.hash(input);
        hasher.reset();
        let hash2 = hasher.hash(input);
        
        assert_eq!(hash1, hash2);
    }
}
