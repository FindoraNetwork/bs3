use alloc::{
    collections::{btree_map::Range, BTreeMap},
    vec::Vec,
};

use crate::Result;

use super::Store;

pub struct MemoryBackend {
    pub cache: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }
}

impl Store for MemoryBackend {
    type Range<'a> = Range<'a, Vec<u8>, Vec<u8>>;

    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
        Ok(match self.cache.get(key) {
            Some(v) => Some(v.as_slice()),
            None => None,
        })
    }

    fn range(&self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range<'_>> {
        Ok(self.cache.range(begin_key..end_key))
    }

    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let inner = &mut self.cache;
        for (key, value) in batch {
            inner.insert(key, value);
        }
        Ok(())
    }

    // fn commit(&mut self) -> Result<()> {
    // let store = self.store.get_mut();
    // self.cache.append(store);
    // Ok(())
    // }
}
