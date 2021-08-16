use core::cell::RefCell;

use alloc::{collections::{BTreeMap, btree_map::Range}, rc::Rc, vec::Vec};

use crate::Result;

use super::Store;

pub struct MemoryBackend {
    store: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl<'a> Store<'a> for MemoryBackend {
    type Range = Range<'a, Vec<u8>, Vec<u8>>;

    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
        let inner = &self.store;
        Ok(match inner.get(key) {
            Some(v) => Some(v.as_slice()),
            None => None
        })
    }
    
    fn range(&'a self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range> {
        let inner = &self.store;
        // change api to reduce copy.
        Ok(inner.range(begin_key .. end_key))
    }

    fn execute(&self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let mut inner = self.store;
        for (key, value) in batch {
            inner.insert(key, value);
        }
        Ok(())
    }
}

