use aes::Aes256;
use ctr::{Ctr64BE, cipher::{KeyIvInit, StreamCipher}};
use rand::{Rng, thread_rng};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::{NekoError, NekoResult, NekoHash, KawaiiHash};

type Aes256Ctr64BE = Ctr64BE<Aes256>;

/// Converts a byte slice to a hexadecimal string
#[inline]
pub fn to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// Converts a hexadecimal string to a byte vector
pub fn from_hex(hex: &str) -> NekoResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(NekoError::EncodingError("Invalid hex string length".into()));
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)
            .map_err(|e| NekoError::EncodingError(format!("Invalid hex character: {}", e)))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

/// Combines multiple hashes into one using a rotating XOR operation
#[inline]
pub fn combine_hashes(hashes: &[Vec<u8>]) -> Vec<u8> {
    if hashes.is_empty() {
        return Vec::new();
    }

    let max_len = hashes.iter().map(|h| h.len()).max().unwrap();
    let mut result = vec![0u8; max_len];

    for hash in hashes {
        for (i, &byte) in hash.iter().enumerate() {
            let idx = i % max_len;
            result[idx] ^= byte;
            result[idx] = result[idx].rotate_left(3);
        }
    }

    result
}

/// Encrypts data using AES-256-CTR with either a provided key or a random key
pub fn encrypt_data(data: &[u8], key: Option<&[u8]>) -> NekoResult<Vec<u8>> {
    let mut rng = thread_rng();
    
    let key = match key {
        Some(k) if k.len() == 32 => k.to_vec(),
        Some(_) => return Err(NekoError::KeyError("Key must be exactly 32 bytes".into())),
        None => {
            let mut key = vec![0u8; 32];
            rng.fill(&mut key[..]);
            key
        }
    };

    let mut iv = [0u8; 16];
    rng.fill(&mut iv[..]);

    let mut cipher = Aes256Ctr64BE::new(key[..].into(), &iv.into());
    let mut buf = data.to_vec();
    cipher.apply_keystream(&mut buf);

    let mut result = Vec::with_capacity(16 + buf.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&buf);
    
    Ok(BASE64.encode(result).into_bytes())
}

/// Decrypts data using AES-256-CTR with the provided key
pub fn decrypt_data(encrypted_data: &[u8], key: &[u8]) -> NekoResult<Vec<u8>> {
    if key.len() != 32 {
        return Err(NekoError::KeyError("Key must be exactly 32 bytes".into()));
    }

    let encrypted = BASE64.decode(encrypted_data)
        .map_err(|e| NekoError::EncodingError(format!("Invalid base64: {}", e)))?;

    if encrypted.len() < 16 {
        return Err(NekoError::CryptoError("Invalid encrypted data".into()));
    }

    let (iv, ciphertext) = encrypted.split_at(16);
    
    let mut cipher = Aes256Ctr64BE::new(key.into(), iv.into());
    let mut buf = ciphertext.to_vec();
    cipher.apply_keystream(&mut buf);

    Ok(buf)
}

/// Generates a random encryption key
#[inline]
pub fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    thread_rng().fill(&mut key[..]);
    key
}

/// Formats a key as a base64 string for easy storage
#[inline]
pub fn key_to_base64(key: &[u8]) -> String {
    BASE64.encode(key)
}

/// Converts a base64 key string back to bytes
pub fn key_from_base64(key_str: &str) -> NekoResult<Vec<u8>> {
    BASE64.decode(key_str)
        .map_err(|e| NekoError::EncodingError(format!("Invalid base64 key: {}", e)))
}

/// Constant-time comparison of two byte slices
/// Useful for comparing hashes without timing attacks
#[inline]
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Performs key stretching using multiple hash iterations
/// Useful for password hashing or key derivation
pub fn stretch_key(data: &[u8], iterations: usize, output_size: usize) -> NekoResult<Vec<u8>> {
    if iterations == 0 {
        return Err(NekoError::InvalidInput("Iterations must be greater than 0".into()));
    }
    if output_size == 0 {
        return Err(NekoError::InvalidInput("Output size must be greater than 0".into()));
    }

    let mut result = data.to_vec();
    let mut hasher = KawaiiHash::with_size(output_size);
    
    for _ in 0..iterations {
        result = hasher.hash(&result);
    }
    
    Ok(result)
}

/// Generates a deterministic key from a password and salt
pub fn derive_key(password: &[u8], salt: &[u8]) -> NekoResult<Vec<u8>> {
    if password.is_empty() {
        return Err(NekoError::InvalidInput("Password cannot be empty".into()));
    }
    if salt.is_empty() {
        return Err(NekoError::InvalidInput("Salt cannot be empty".into()));
    }

    let mut input = Vec::with_capacity(password.len() + salt.len());
    input.extend_from_slice(password);
    input.extend_from_slice(salt);
    
    stretch_key(&input, 10000, 32)
}

/// Generates a cryptographically secure random salt
#[inline]
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    thread_rng().fill(&mut salt[..]);
    salt
}

/// Performs a time-based key derivation
/// Useful for time-sensitive operations or temporary keys
pub fn time_based_key(seed: &[u8], time_window: u64) -> NekoResult<Vec<u8>> {
    if seed.is_empty() {
        return Err(NekoError::InvalidInput("Seed cannot be empty".into()));
    }
    if time_window == 0 {
        return Err(NekoError::InvalidInput("Time window must be greater than 0".into()));
    }

    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| NekoError::HashError(format!("System time error: {}", e)))?
        .as_secs();
    
    let time_slot = now / time_window;
    let mut input = Vec::with_capacity(seed.len() + 8);
    input.extend_from_slice(seed);
    input.extend_from_slice(&time_slot.to_le_bytes());
    
    derive_key(&input, &generate_salt())
}

/// Rotates a key by a specified number of bits
#[inline]
pub fn rotate_key(key: &[u8], bits: u32) -> Vec<u8> {
    let mut result = key.to_vec();
    let total_bits = key.len() * 8;
    let rotation = bits as usize % total_bits;
    
    if rotation == 0 {
        return result;
    }
    
    let bytes_to_rotate = rotation / 8;
    let remaining_bits = rotation % 8;
    
    if bytes_to_rotate > 0 {
        result.rotate_left(bytes_to_rotate);
    }
    
    if remaining_bits > 0 {
        let mut carry = 0u8;
        for byte in result.iter_mut() {
            let new_carry = *byte >> (8 - remaining_bits);
            *byte = (*byte << remaining_bits) | carry;
            carry = new_carry;
        }
        result[0] |= carry;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 2, 3, 5];
        
        assert!(constant_time_compare(&a, &b));
        assert!(!constant_time_compare(&a, &c));
    }

    #[test]
    fn test_key_stretching() {
        let data = b"password123";
        let result = stretch_key(data, 1000, 32).unwrap();
        assert_eq!(result.len(), 32);
        
        assert!(stretch_key(data, 0, 32).is_err());
        assert!(stretch_key(data, 1000, 0).is_err());
    }

    #[test]
    fn test_key_derivation() {
        let password = b"password123";
        let salt = generate_salt();
        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key1, key2);
        
        assert!(derive_key(&[], &salt).is_err());
        assert!(derive_key(password, &[]).is_err());
    }

    #[test]
    fn test_time_based_key() {
        let seed = b"test_seed";
        let key1 = time_based_key(seed, 30).unwrap();
        let key2 = time_based_key(seed, 30).unwrap();
        
        assert_eq!(key1.len(), 32);
        assert!(time_based_key(&[], 30).is_err());
        assert!(time_based_key(seed, 0).is_err());
    }

    #[test]
    fn test_key_rotation() {
        let key = vec![0b10101010, 0b11110000];
        let rotated = rotate_key(&key, 4);
        assert_eq!(rotated, vec![0b10101111, 0b00001010]);
    }

    #[test]
    fn test_hex_conversion() {
        let original = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let hex = to_hex(&original);
        let decoded = from_hex(&hex).unwrap();
        assert_eq!(original, decoded);
        
        assert!(from_hex("invalid").is_err());
        assert!(from_hex("deadbeef1").is_err());
    }

    #[test]
    fn test_encryption() {
        let data = b"test data";
        let key = generate_key();
        
        let encrypted = encrypt_data(data, Some(&key)).unwrap();
        let decrypted = decrypt_data(&encrypted, &key).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
        
        let wrong_key = generate_key();
        let wrong_decrypted = decrypt_data(&encrypted, &wrong_key);
        assert!(wrong_decrypted.is_ok() && wrong_decrypted.unwrap() != data);
    }
}
