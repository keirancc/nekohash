/// Converts a byte slice to a hexadecimal string
pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Converts a hexadecimal string to a byte vector
pub fn from_hex(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Invalid hex string length".to_string());
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex character: {}", e))
        })
        .collect()
}

/// Combines multiple hashes into one using a kawaii mixing function
pub fn combine_hashes(hashes: &[Vec<u8>]) -> Vec<u8> {
    if hashes.is_empty() {
        return Vec::new();
    }

    let max_len = hashes.iter().map(|h| h.len()).max().unwrap();
    let mut result = vec![0u8; max_len];

    for hash in hashes {
        for (i, &byte) in hash.iter().enumerate() {
            result[i % max_len] ^= byte;
            result[i % max_len] = result[i % max_len].rotate_left(3);
        }
    }

    result
}
