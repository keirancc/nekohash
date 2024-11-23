use aes::Aes256;
use ctr::{Ctr64BE, cipher::{KeyIvInit, StreamCipher}};
use rand::{Rng, thread_rng};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

type Aes256Ctr64BE = Ctr64BE<Aes256>;

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

/// Encrypts data using AES-256-CTR with either a provided key or a random key
pub fn encrypt_data(data: &[u8], key: Option<&[u8]>) -> Result<Vec<u8>, String> {
    let mut rng = thread_rng();
    
    // Generate or use provided key
    let key = match key {
        Some(k) if k.len() == 32 => k.to_vec(),
        Some(_) => return Err("Key must be exactly 32 bytes".to_string()),
        None => {
            let mut key = vec![0u8; 32];
            rng.fill(&mut key[..]);
            key
        }
    };

    // Generate IV
    let mut iv = [0u8; 16];
    rng.fill(&mut iv[..]);

    // Encrypt
    let mut cipher = Aes256Ctr64BE::new(key[..].into(), &iv.into());
    let mut buf = data.to_vec();
    cipher.apply_keystream(&mut buf);

    // Format: base64(iv + ciphertext)
    let mut result = Vec::with_capacity(16 + buf.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&buf);
    
    Ok(BASE64.encode(result).into_bytes())
}

/// Decrypts data using AES-256-CTR with the provided key
pub fn decrypt_data(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be exactly 32 bytes".to_string());
    }

    // Decode base64
    let encrypted = BASE64.decode(encrypted_data)
        .map_err(|e| format!("Invalid base64: {}", e))?;

    if encrypted.len() < 16 {
        return Err("Invalid encrypted data".to_string());
    }

    // Split IV and ciphertext
    let (iv, ciphertext) = encrypted.split_at(16);
    
    // Decrypt
    let mut cipher = Aes256Ctr64BE::new(key.into(), iv.into());
    let mut buf = ciphertext.to_vec();
    cipher.apply_keystream(&mut buf);

    Ok(buf)
}

/// Generates a random encryption key
pub fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    thread_rng().fill(&mut key[..]);
    key
}

/// Formats a key as a base64 string for easy storage
pub fn key_to_base64(key: &[u8]) -> String {
    BASE64.encode(key)
}

/// Converts a base64 key string back to bytes
pub fn key_from_base64(key_str: &str) -> Result<Vec<u8>, String> {
    BASE64.decode(key_str)
        .map_err(|e| format!("Invalid base64 key: {}", e))
}
