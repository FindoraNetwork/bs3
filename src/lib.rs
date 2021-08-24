#![feature(generic_associated_types)]
#![feature(cell_leak)]
#![no_std]

/// For features and alloc.
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

mod operation;
pub use operation::{Operation, OperationBytes};

mod cow_lite;
pub use cow_lite::Cow;

mod error;
pub use error::{Error, Result};

mod model;
pub use model::Value;

mod prelude;

