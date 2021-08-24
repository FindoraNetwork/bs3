use crate::{bytes_ref::BytesRef, Result};
use alloc::vec::Vec;

pub trait Store {
    #[cfg(feature = "nightly")]
    type Range<'a>: Iterator<Item = (BytesRef<'a>, BytesRef<'a>)>;

    #[cfg(feature = "nightly")]
    /// Provide this method to range key.
    fn range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Self::Range<'_>>;

    /// Provide this method to execute transaction.
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    #[cfg(not(feature = "nightly"))]
    fn get_lt(&self, key: &[u8]) -> Result<Option<BytesRef<'_>>>;

    #[cfg(feature = "nightly")]
    fn get_lt(&self, key: &[u8]) -> Result<Option<BytesRef<'_>>> {
        let mut value = self.range(&[], key)?;
        Ok(match value.next() {
            Some((_, v)) => Some(v),
            None => None,
        })
    }
}
