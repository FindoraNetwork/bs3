use alloc::vec::Vec;

use crate::{
    types::{BranchName, VersionName},
    Result,
};

const STORE_TYPE_BRANCH_PRFIX: u8 = 0x01;
const STORE_TYPE_KEY_PRFIX: u8 = 0x02;
const STORE_TYPE_DATA_PRFIX: u8 = 0x03;
const STORE_TYPE_VERSION_PRFIX: u8 = 0x04;

pub fn data_store_key(branch_id: u64, key_id: u64, version_id: u64) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.push(STORE_TYPE_DATA_PRFIX);
    bytes.extend_from_slice(&branch_id.to_be_bytes());

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

pub fn branch_store_key(branch_name: &BranchName) -> Result<Vec<u8>> {
    let mut res = Vec::new();

    res.push(STORE_TYPE_BRANCH_PRFIX);

    let len: u8 = branch_name.0.len().try_into()?;

    res.push(len);
    res.extend_from_slice(&branch_name.0);

    Ok(res)
}

pub fn version_store_key(branch_id: u64, version_name: &VersionName) -> Result<Vec<u8>> {
    let mut res = Vec::new();

    res.push(STORE_TYPE_VERSION_PRFIX);

    res.extend_from_slice(&branch_id.to_be_bytes());

    let len: u8 = version_name.0.len().try_into()?;
    res.push(len);
    res.extend_from_slice(&version_name.0);

    Ok(res)
}
