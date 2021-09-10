use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{Error, OperationBytes, Result, utils::cbor_encode};
use ciborium::de::from_reader;

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
    pub operation: OperationBytes,
}

impl FromStoreBytes for StoreValue {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = from_reader(bytes).map_err(|_| Error::CborDeIoError)?;
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
    pub height: i64,
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
        let r = from_reader(bytes).map_err(|_| Error::CborDeIoError)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreType {
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
        let r = from_reader(bytes).map_err(|_| Error::CborDeIoError)?;
        Ok(r)
    }
}
