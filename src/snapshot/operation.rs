use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    // Read(u64),
    Update(Vec<u8>),
    Delete,
}
