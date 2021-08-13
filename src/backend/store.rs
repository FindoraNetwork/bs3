use crate::Result;
use alloc::vec::Vec;

pub trait Store {
    /// Provide this method to get value from backend.
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>>;

    /// Provide this method to execute transaction.
    fn execute(&self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    /// Provide this method to get key.
    fn get_lt(&self, key: &[u8]) -> Result<Option<&[u8]>>;
}
