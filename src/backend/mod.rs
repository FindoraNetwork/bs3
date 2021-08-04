mod store;
pub use store::{Operation, Store};

#[cfg(features = "sled-backend")]
mod sled;

