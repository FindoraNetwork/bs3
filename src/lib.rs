#![feature(generic_associated_types)]
#![feature(cell_leak)]
#![no_std]

extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

pub mod backend;

mod transaction;
pub use transaction::Transaction;

pub mod prelude;

mod error;
pub use error::{Error, Result};

mod snapshot;
pub use snapshot::SnapshotableStorage;

pub mod bytes_ref;

