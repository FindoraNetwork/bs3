use alloc::string::ToString;
use alloc::vec::Vec;
use ciborium::de::from_reader;
use crate::{utils::cbor_encode, Error, OperationBytes, Result};
use crate::snapshot::{FromStoreBytes, ToStoreBytes};

#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MerkleValue {
    pub operation: OperationBytes,
}

impl FromStoreBytes for MerkleValue {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = from_reader(bytes).map_err(|e| Error::CborDeIoError(e.to_string()))?;
        Ok(r)
    }
}

#[cfg(feature = "cbor")]
impl ToStoreBytes for MerkleValue {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = cbor_encode(self)?;
        Ok(bytes)
    }
}