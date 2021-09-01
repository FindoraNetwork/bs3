use crate::{Cow, Result};
use alloc::vec::Vec;

pub trait Tree {
    /// Get value by key in tree.
    fn get(&self, key: &[u8]) -> Result<Option<Cow<'_, Vec<u8>>>>;
    /// Get value mut.

    fn get_mut(&mut self, key: &[u8]) -> Result<Option<&mut [u8]>>;

    /// Insert value.
    fn insert(&mut self, key: &[u8], value: Vec<u8>) -> Result<Option<Vec<u8>>>;

    /// Remove value.
    fn remove(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}
