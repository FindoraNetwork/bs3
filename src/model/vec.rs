//!
//! vec cache layer
//!
use core::{fmt::Debug, mem};

use crate::model::{Model, ValueType};
use crate::{Operation, OperationBytes};
use alloc::{collections::BTreeMap, vec::Vec as AllocVec};
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub const INDEX_VEC_LEN: u64 = u64::MAX;

/// define vec,inner value is btree
///     key : usize
#[derive(Debug, Clone)]
pub struct Vec<T: ValueType> {
    cache: BTreeMap<u64, Operation<T>>,
    current_len: Option<u64>,
}

impl<T: ValueType> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_len(&mut self, len: u64) {
        self.current_len = Some(len);
    }

    pub fn len(&self) -> Option<u64> {
        self.current_len
    }

    pub fn insert_operation(&mut self, index: u64, op: Operation<T>) -> Option<Operation<T>> {
        self.cache.insert(index, op)
    }

    pub fn remove_operation(&mut self, index: u64) -> Option<Operation<T>> {
        self.cache.remove(&index)
    }

    pub fn contains_key(&self, index: &u64) -> bool {
        self.cache.contains_key(index)
    }

    pub fn get(&self, index: &u64) -> Option<&Operation<T>> {
        self.cache.get(index)
    }

    pub fn get_mut(&mut self, index: &u64) -> Option<&mut Operation<T>> {
        self.cache.get_mut(index)
    }
}

impl<T: ValueType> Default for Vec<T> {
    fn default() -> Self {
        Self {
            cache: BTreeMap::new(),
            current_len: None,
        }
    }
}

/// impl model
impl<T: ValueType> Model for Vec<T> {
    /// Consume the data in the cache
    /// Also convert key to vec<u8>
    fn operations(&mut self) -> crate::Result<AllocVec<(AllocVec<u8>, OperationBytes)>> {
        use crate::utils::cbor_encode;

        let mut res = AllocVec::new();

        let values = mem::replace(&mut self.cache, BTreeMap::new());

        for (k, v) in values.into_iter() {
            let key = cbor_encode(&k)?;
            let value = v.to_bytes()?;
            res.push((key, value));
        }
        if let Some(l) = self.current_len.take() {
            res.push((
                cbor_encode(&INDEX_VEC_LEN)?,
                Operation::Update(l).to_bytes()?,
            ));
        }
        Ok(res)
    }

    /// define type 2
    fn type_code(&self) -> u32 {
        2
    }

    /// Merge two caches
    fn merge(&mut self, mut other: Self) {
        //[TODO] may conflict
        self.cache.append(&mut other.cache);
        if let Some(l) = other.len() {
            self.current_len = Some(l);
        }
    }
}
