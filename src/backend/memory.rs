use std::sync::Mutex;

use alloc::{collections::{BTreeMap, btree_map::Range},vec::Vec};

use alloc::sync::Arc;

use crate::{Error, Result};

use super::Store;

pub struct MemoryBackend {
    cache: BTreeMap<Vec<u8>, Vec<u8>>,
    store: Arc<Mutex<BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl MemoryBackend {
    // type Range = Range<'a, Vec<u8>, Vec<u8>>;

    pub fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
        Ok(match self.cache.get(key) {
            Some(v) => Some(v.as_slice()),
            None => None
        })
    }

   //  fn range(&'a self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range> {
        // // let inner = self.store.lock().map_err(|e| {Error::LockReadError})?;
        // Ok(self.store.lock().unwrap().range(begin_key .. end_key))
   //  }

    pub fn execute(&self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let mut inner = self.store.lock().map_err(|_| {Error::LockReadError})?;
        for (key, value) in batch {
            inner.insert(key, value);
        }
        Ok(())
    }
}

