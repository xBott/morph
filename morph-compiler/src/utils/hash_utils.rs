use sha2::{Sha256, Digest};

pub fn hash_str_to_i8_positive(value: &str) -> i8 {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let hash = hasher.finalize();

    let byte = hash[0];
    (byte & 0x7F) as i8
}


