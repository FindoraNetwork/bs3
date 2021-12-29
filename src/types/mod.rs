mod store_value;
pub use store_value::StoreValue;

mod operation;
pub use operation::*;

mod store_key;
pub use store_key::*;

use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct BranchName(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct VersionName(pub Vec<u8>);
