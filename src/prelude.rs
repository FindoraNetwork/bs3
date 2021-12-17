use crate::Result;
use alloc::vec::Vec;

pub trait Tree {
    /// Get value by key in tree.
    fn tree_get(&self, key: &Vec<u8>, height: i64) -> Result<Vec<u8>>;
}
