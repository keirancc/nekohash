use crate::NekoHash;
use rand::{Rng, thread_rng};

/// TsundereHash implementation with fixed 32-byte output
pub struct TsundereHash {
    rounds: usize,
    state: [u64; 4],
}

impl Default for TsundereHash {
    fn default() -> Self {
        Self::new()
    }
}

impl TsundereHash {
    /// Creates a new TsundereHash with default settings
    pub fn new() -> Self {
        Self {
            rounds: 8,
            state: [0; 4],
        }
    }

    /// Creates a new TsundereHash with specified number of rounds
    pub fn with_rounds(rounds: usize) -> Self {
        Self {
            rounds,
            state: [0; 4],
        }
    }
}

impl NekoHash for TsundereHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut state = self.state;
        let mut result = Vec::with_capacity(32);

        // Initial state mixing
        for chunk in data.chunks(32) {
            for (i, &byte) in chunk.iter().enumerate() {
                let state_idx = (i / 8) % 4;
                state[state_idx] ^= (byte as u64) << ((i % 8) * 8);
            }

            // Multiple rounds of transformation
            for _ in 0..self.rounds {
                // Tsundere transformations
                state[0] = state[0].wrapping_add(state[1]);
                state[1] = state[1].rotate_left(13);
                state[2] = state[2].wrapping_sub(state[3]);
                state[3] = state[3].rotate_right(7);

                state[0] ^= state[2];
                state[1] ^= state[3];
                
                // Add some randomness (because tsundere is unpredictable)
                let random = thread_rng().gen::<u64>();
                state[thread_rng().gen_range(0..4)] ^= random;
            }
        }

        // Final mixing
        for &s in &state {
            result.extend_from_slice(&s.to_le_bytes());
        }

        result
    }

    fn output_size(&self) -> usize {
        32
    }

    fn reset(&mut self) {
        self.state = [0; 4];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsundere_hash() {
        let hasher = TsundereHash::new();
        let data = b"Hello, Tsundere World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_tsundere_hash_custom_rounds() {
        let hasher = TsundereHash::with_rounds(16);
        let data = b"Hello, Tsundere World!";
        let hash = hasher.hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_tsundere_hash_reset() {
        let mut hasher = TsundereHash::new();
        let data = b"Hello, Tsundere World!";
        let hash1 = hasher.hash(data);
        hasher.reset();
        let hash2 = hasher.hash(data);
        assert_eq!(hash1, hash2);
    }
}
