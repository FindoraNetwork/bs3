use crate::Result;
use alloc::vec::Vec;

pub trait Tree {
    /// Get value by key in tree.
    fn tree_get(&self, key: &[u8]) -> Result<Vec<u8>>;
}
