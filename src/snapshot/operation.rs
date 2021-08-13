use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation<'a> {
    // Read(u64),
    Update(&'a [u8]),
    Delete,
}

impl<'a> Operation<'a> {
    pub fn to_operation_owned(self) -> OperationOwned {
        match self {
            Operation::Update(v) => OperationOwned::Update(Vec::from(v)),
            Operation::Delete => OperationOwned::Delete,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationOwned {
    // Read(u64),
    Update(Vec<u8>),
    Delete,
}

impl OperationOwned {
    pub fn to_operation(&self) -> Operation<'_> {
        match self {
            OperationOwned::Update(v) => Operation::Update(v.as_slice()),
            OperationOwned::Delete => Operation::Delete,
        }
    }
}
