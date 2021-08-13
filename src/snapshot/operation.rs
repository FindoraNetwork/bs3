use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation<'a> {
    // Read(u64),
    Update(&'a [u8]),
    Delete,
}
