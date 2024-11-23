pub mod kawaii;
pub mod tsundere;
pub mod magical;
pub mod utils;

pub use kawaii::KawaiiHash;
pub use tsundere::TsundereHash;
pub use magical::MagicalHash;

/// The main trait that all Nekohash algorithms implement
pub trait NekoHash {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    fn hash_size(&self) -> usize;
    fn algorithm_name(&self) -> &'static str;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
