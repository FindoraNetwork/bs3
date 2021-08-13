use crate::Result;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    // Read(u64),
    Update(Vec<u8>),
    Delete,
}

pub trait Store {
    /// Provide this method to get value from backend.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Provide this method to execute transaction.
    fn execute(&self, batch: Vec<(Vec<u8>, Operation)>) -> Result<()>;

    fn get_lt(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}
