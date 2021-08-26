use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Result;

// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
// pub enum Operation<'a> {
//     // Read(u64),
//     Update(&'a [u8]),
//     Delete,
// }
//
// impl<'a> Operation<'a> {
//     pub fn to_operation_owned(self) -> OperationOwned {
//         match self {
//             Operation::Update(v) => OperationOwned::Update(Vec::from(v)),
//             Operation::Delete => OperationOwned::Delete,
//         }
//     }
// }
//
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation<T> {
    // Read(u64),
    Update(T),
    Delete,
}

impl<T> Operation<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn to_bytes(&self) -> Result<OperationBytes> {
        match self {
            Operation::Update(v) => Ok(OperationBytes::Update(serde_cbor::to_vec(v)?)),
            Operation::Delete => Ok(OperationBytes::Delete),
        }
    }

    pub fn from_bytes(bytes: &OperationBytes) -> Result<Self> {
        Ok(match bytes {
            Operation::Update(v) => Operation::Update(serde_cbor::from_slice(&v)?),
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
