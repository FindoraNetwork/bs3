use alloc::{
    collections::{btree_map::Range, BTreeMap},
    vec::Vec,
};

use crate::{CowBytes, Result};

use core::ops::Bound::{Excluded, Included};

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

pub struct MemoryRange<'a> {
    pub v: Range<'a, Vec<u8>, Vec<u8>>,
}

impl<'a> Iterator for MemoryRange<'a> {
    type Item = (CowBytes<'a>, CowBytes<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.v
            .next()
            .map(|v| (CowBytes::Borrowed(v.0), CowBytes::Borrowed(v.1)))
    }
}

impl<'a> DoubleEndedIterator for MemoryRange<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.v
            .next_back()
            .map(|v| (CowBytes::Borrowed(v.0), CowBytes::Borrowed(v.1)))
    }
}

impl Store for MemoryBackend {
    type Range<'a> = MemoryRange<'a>;

    fn range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Self::Range<'_>> {
        Ok(MemoryRange {
            v: self
                .cache
                .range((Excluded(Vec::from(begin_key)), Included(Vec::from(end_key)))),
        })
    }

    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let inner = &mut self.cache;
        for (key, value) in batch {
            inner.insert(key, value);
        }
        Ok(())
    }
}
