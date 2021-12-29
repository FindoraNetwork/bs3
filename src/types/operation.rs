//!
//! All written values will become operation
//! if value exist
//!     T => Operation::Update(T)
//! if value removed
//!     Operation::Update(T) => Operation::Delete
//!

use alloc::vec::Vec;

use crate::{
    prelude::{FromBytes, ToBytes},
    Result,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation<T: FromBytes + ToBytes> {
    Update(T),
    Delete,
}

impl<T> Operation<T>
where
    T: FromBytes + ToBytes,
{
    pub fn to_bytes(&self) -> Result<OperationBytes> {
        Ok(match self {
            Operation::Update(v) => OperationBytes::Update(v.to_bytes()?),
            Operation::Delete => OperationBytes::Delete,
        })
    }

    pub fn from_bytes(bytes: &OperationBytes) -> Result<Self> {
        Ok(match bytes {
            Operation::Update(v) => Operation::Update(T::from_bytes(v)?),
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
