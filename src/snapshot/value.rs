use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Result;

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
    pub operation: Operation,
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for StoreValue {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = serde_cbor::to_vec(self)?;
        Ok(bytes)
    }
}

#[cfg(feature = "cbor")]
impl FromStoreBytes for StoreValue {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let r = serde_cbor::from_slice(bytes)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreHeight {
    pub height: u64,
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for StoreHeight {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = serde_cbor::to_vec(self)?;
        Ok(bytes)
    }
}

#[cfg(feature = "cbor")]
impl FromStoreBytes for StoreHeight {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let r = serde_cbor::from_slice(bytes)?;
        Ok(r)
    }
}
