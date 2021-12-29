use alloc::vec::Vec;

use crate::Result;

const STORE_TYPE_BRANCH_PRFIX: u8 = 0x01;
const STORE_TYPE_KEY_PRFIX: u8 = 0x02;

const STORE_SUBTYPE_ID_PREFIX: u8 = 0x11;

pub fn data_store_key(branch_id: u64, key_id: u64, version_id: u64) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.push(STORE_TYPE_BRANCH_PRFIX);
    bytes.extend_from_slice(&branch_id.to_be_bytes());

    bytes.push(STORE_SUBTYPE_ID_PREFIX);
    bytes.extend_from_slice(&key_id.to_be_bytes());

    bytes.extend_from_slice(&version_id.to_be_bytes());

    bytes
}

pub fn key_store_key(branch_id: u64, key: &[u8]) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    bytes.push(STORE_TYPE_KEY_PRFIX);
    bytes.extend_from_slice(&branch_id.to_be_bytes());

    let key_len: u8 = key.len().try_into()?;

    bytes.push(key_len);
    bytes.extend_from_slice(key);

    Ok(bytes)
}
