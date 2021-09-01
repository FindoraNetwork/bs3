use alloc::vec::Vec;

#[cfg(feature = "cbor")]
use minicbor::{Decode as Deserialize, Encode as Serialize};

use crate::{utils::cbor_encode, OperationBytes, Result};

pub trait ToStoreBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait FromStoreBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct StoreValue {
    #[n(0)]
    pub operation: OperationBytes,
}

impl FromStoreBytes for StoreValue {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = minicbor::decode(bytes)?;
        Ok(r)
    }
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for StoreValue {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = cbor_encode(self)?;
        Ok(bytes)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreHeight {
    #[n(0)]
    pub height: u64,
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for StoreHeight {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = cbor_encode(self)?;
        Ok(bytes)
    }
}

#[cfg(feature = "cbor")]
impl FromStoreBytes for StoreHeight {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = minicbor::decode(bytes)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreType {
    #[n(0)]
    pub ty: u32,
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for StoreType {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = cbor_encode(self)?;
        Ok(bytes)
    }
}

#[cfg(feature = "cbor")]
impl FromStoreBytes for StoreType {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = minicbor::decode(bytes)?;
        Ok(r)
    }
}
