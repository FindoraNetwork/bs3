use crate::{Result, bytes_ref::BytesRef};
use alloc::vec::Vec;

pub trait Store {
    type Range: Iterator<Item = (Vec<u8>, Vec<u8>)>;

    /// Provide this method to range key.
    fn range(&self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range>;

    /// Provide this method to execute transaction.
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;
}
