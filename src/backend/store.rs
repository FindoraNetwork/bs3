//!
//! Trait Store is storage layer implementation constraints
//! The abstracted range is supplied to the parent class Store to call
//! where the type specifies the return of the range
//!

use crate::{CowBytes, Result};
use alloc::vec::Vec;

pub trait Store: Send + Sync + Clone {
    /// Provide this method to execute transaction.
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let mut vec = Vec::new();
        vec.push((key, value));
        self.execute(vec)
    }

    /// This is the upgraded version of get_ge
    /// The main thing is that the start index is not a fixed empty vec
    fn get_in(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Option<CowBytes<'_>>>;

    fn get(&self, key: &[u8]) -> Result<Option<CowBytes<'_>>>;
}
