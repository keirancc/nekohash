use crate::NekoHash;

/// TsundereHash is a tsundere-themed hashing algorithm that initially seems
/// hostile but actually produces reliable hashes
pub struct TsundereHash {
    rounds: usize,
}

impl Default for TsundereHash {
    fn default() -> Self {
        Self::new()
    }
}

impl TsundereHash {
    pub fn new() -> Self {
        Self { rounds: 3 }
    }

    pub fn with_rounds(rounds: usize) -> Self {
        Self { rounds }
    }
}

impl NekoHash for TsundereHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(32);
        let mut state = [0u8; 32];

        for (i, &byte) in data.iter().enumerate() {
            state[i % 32] ^= byte;
            state[(i + 7) % 32] = state[(i + 7) % 32].wrapping_add(byte);
            
            if i % 2 == 0 {
                state[i % 32] = state[i % 32].rotate_left(3);
            } else {
                state[i % 32] = state[i % 32].rotate_right(2);
            }
        }

        for _round in 0..self.rounds {
            for i in 0..32 {
                let prev = state[(i + 31) % 32];
                let next = state[(i + 1) % 32];
                state[i] ^= prev.wrapping_add(next);
                state[i] = state[i].wrapping_mul(0x1B);
            }
        }

        result.extend_from_slice(&state);
        result
    }

    fn hash_size(&self) -> usize {
        32
    }

    fn algorithm_name(&self) -> &'static str {
        "TsundereHash (๑•̀ㅁ•́๑)✧"
    }
}
