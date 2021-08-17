use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Result;

use super::Operation;

pub trait ToStoreBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

pub trait FromStoreBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct StoreValue<'a> {
    #[serde(borrow)]
    pub operation: Operation<'a>,
}

impl<'a> StoreValue<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        let r = serde_cbor::from_slice(bytes)?;
        Ok(r)
    }
}

#[cfg(feature = "cbor")]
impl<'a> ToStoreBytes for StoreValue<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = serde_cbor::to_vec(self)?;
        Ok(bytes)
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
