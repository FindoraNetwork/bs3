use crate::Result;
use alloc::vec::Vec;

pub trait Store<'a> {
    type Range: Iterator<Item = (&'a Vec<u8>, &'a Vec<u8>)>;

    // wait GAT stable to reduce lifetime.
    // type Range<'a>: Iterator<Item = (&'a [u8], &'a [u8])>;

    /// Provide this method to get value from backend.
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>>;

    /// Provide this method to execute transaction.
    fn execute(&self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    /// Provide this method to range key.
    fn range(&'a self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range>;
}
