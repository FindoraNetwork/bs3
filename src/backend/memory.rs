//!
//! Storage layer in the form of memory
//!

use alloc::{
    collections::{btree_map::Range, BTreeMap},
    vec::Vec,
};

use crate::{CowBytes, Result};

use core::{fmt, ops::Bound::Included};

use super::Store;

///
/// The data is finally stored in the B-tree
/// key:Vec<u8>
///     like:
///         bytes({name_space}-ty)
///         bytes({name_space}-kw-{hex(1)}-{:00000000000000000001})
/// value:Vec<u8>
///     like:
///         bytes(Operation::Update(1))
///         Bytes(Operation::Remove)
///
pub struct MemoryBackend {
    pub cache: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl fmt::Debug for MemoryBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in &self.cache {
            f.write_fmt(format_args!(
                "key: {}, value: {}\n",
                hex::encode(key),
                hex::encode(value)
            ))?;
        }
        Ok(())
    }
}

impl MemoryBackend {
    /// create MemoryBackend
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }
}

/// Range used to host the b-tree
pub struct MemoryRange<'a> {
    pub v: Range<'a, Vec<u8>, Vec<u8>>,
}

/// impl Iterator
impl<'a> Iterator for MemoryRange<'a> {
    type Item = (CowBytes<'a>, CowBytes<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.v
            .next()
            .map(|v| (CowBytes::Borrowed(v.0), CowBytes::Borrowed(v.1)))
    }
}

/// impl DoubleEndedIterator
impl<'a> DoubleEndedIterator for MemoryRange<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.v
            .next_back()
            .map(|v| (CowBytes::Borrowed(v.0), CowBytes::Borrowed(v.1)))
    }
}

/// impl store
/// fn range used in get_ge
impl Store for MemoryBackend {
    type Range<'a> = MemoryRange<'a>;

    /// Search Scope
    fn range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Self::Range<'_>> {
        Ok(MemoryRange {
            v: self
                .cache
                .range((Included(Vec::from(begin_key)), Included(Vec::from(end_key)))),
        })
    }

    /// Batch insert
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let inner = &mut self.cache;
        for (key, value) in batch {
            inner.insert(key, value);
        }
        Ok(())
    }
}
