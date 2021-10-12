//!
//! snapshot utils
//!

use alloc::{format, string::String, vec::Vec};

/// Build current block high key
pub fn current_height_key(namespace: &String) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-ch", namespace).into_bytes()
}

/// Build key
pub fn storage_key<T: AsRef<[u8]>>(namespace: &str, key: T, height: i64) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-kw-{}-{:020}", namespace, hex::encode(key), height).into_bytes()
}

/// Build type key
pub fn type_key(namespace: &String) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-ty", namespace).into_bytes()
}

/// build merkle root key
pub fn merkle_key(namespace: &str, height: i64) -> Vec<u8> {
    format!("{}-mr-{:020}", namespace, height).into_bytes()
}
//
