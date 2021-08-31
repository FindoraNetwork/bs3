use crate::{CowBytes, Result};
use alloc::vec::Vec;

pub trait Store: Send + Sync {
    #[cfg(feature = "nightly")]
    type Range<'a>: DoubleEndedIterator<Item = (CowBytes<'a>, CowBytes<'a>)>;

    #[cfg(feature = "nightly")]
    /// Provide this method to range key.
    fn range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Self::Range<'_>>;

    /// Provide this method to execute transaction.
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()>;

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let mut vec = Vec::new();
        vec.push((key, value));
        self.execute(vec)
    }

    #[cfg(not(feature = "nightly"))]
    fn get_ge(&self, key: &[u8]) -> Result<Option<CowBytes<'_>>>;

    #[cfg(feature = "nightly")]
    fn get_ge(&self, key: &[u8]) -> Result<Option<CowBytes<'_>>> {
        let mut value = self.range(&Vec::new(), key)?;
        Ok(match value.next_back() {
            Some((_, v)) => Some(v),
            None => None,
        })
    }
}
