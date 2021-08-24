use alloc::vec::Vec;

use crate::{BytesRef, Result};

// mod operation;
// pub use operation::{Operation, OperationOwned};

// pub mod utils;

// mod value;
// pub use value::{FromStoreBytes, StoreHeight, StoreValue, ToStoreBytes};

// mod storage;
// pub use storage::SnapshotableStorage;

pub trait KVStore {
    fn get(&self) -> Result<Option<BytesRef<'_>>>;
}
