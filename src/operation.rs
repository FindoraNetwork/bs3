use alloc::vec::Vec;

#[cfg(feature = "cbor")]
use minicbor::{Encode as Serialize, Decode as Deserialize};

use crate::{Result, utils::cbor_encode};

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
    #[n(0)]
    Update(#[n(0)]T),
    #[n(1)]
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
            Operation::Update(v) => Operation::Update(minicbor::decode(v)?),
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
