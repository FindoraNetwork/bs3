//!
//! All written values will become operation
//! if value exist
//!     T => Operation::Update(T)
//! if value removed
//!     Operation::Update(T) => Operation::Delete
//!

use alloc::vec::Vec;

#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

use crate::{utils::cbor_encode, Error, Result};
use alloc::string::ToString;
use ciborium::de::from_reader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation<T> {
    Update(T),
    Delete,
}

impl<T> Operation<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn to_bytes(&self) -> Result<OperationBytes> {
        Ok(match self {
            Operation::Update(v) => OperationBytes::Update(cbor_encode(v)?),
            Operation::Delete => OperationBytes::Delete,
        })
    }

    pub fn from_bytes(bytes: &OperationBytes) -> Result<Self> {
        Ok(match bytes {
            Operation::Update(v) => Operation::Update(
                from_reader(v.as_slice()).map_err(|e| Error::CborDeIoError(e.to_string()))?,
            ),
            Operation::Delete => Operation::Delete,
        })
    }
}

pub type OperationBytes = Operation<Vec<u8>>;

// impl OperationOwned {
// pub fn to_operation(&self) -> Operation<'_> {
//     match self {
//         OperationOwned::Update(v) => Operation::Update(v.as_slice()),
//         OperationOwned::Delete => Operation::Delete,
//     }
// }
// }
