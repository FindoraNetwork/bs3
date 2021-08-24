use alloc::{format, string::String, vec::Vec};

pub fn current_height_key(namespace: &String) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-ch", namespace).into_bytes()
}

pub fn storage_key<T: AsRef<[u8]>>(namespace: &str, key: T, height: u64) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-kwh-{}-{:020}", namespace, hex::encode(key), height).into_bytes()
}

pub fn type_key(namespace: &String) -> Vec<u8> {
    // TODO: use binary key to optimization performance
    format!("{}-ty", namespace).into_bytes()
}
