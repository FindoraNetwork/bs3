//!
//! vec cache layer
//! Don't use it with length that is larger than i64::MAX.
use core::{fmt::Debug, mem};

use crate::model::{Model, ValueType};
use crate::{Operation, OperationBytes};
use alloc::{collections::BTreeMap, vec::Vec as AllocVec};

pub const INDEX_VEC_LEN: u64 = u64::MAX;

/// define vec,inner value is btree
///     key : usize
#[derive(Debug, Clone)]
pub struct Vec<T: ValueType> {
    cache: BTreeMap<u64, Operation<T>>,
    offset: i64,
    //cache of length, avoid frequently access to database.
    len: Option<u64>,
}

impl<T: ValueType> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_offset(&mut self) {
        self.offset += 1;
    }

    pub fn sub_offset(&mut self) {
        self.offset -= 1;
    }

    pub fn len(&self) -> Option<u64> {
        self.len
    }

    pub fn set_length(&mut self, len: u64) {
        self.len = Some(len);
    }

    pub fn offset(&self) -> i64 {
        self.offset
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
            offset: 0,
            len: None,
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

        if let Some(l) = self.len.take() {
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

    /// Merge two caches, may conflict.
    fn merge(&mut self, mut other: Self) {
        let others_offset = other.offset;
        if others_offset == 0 {
            //means other didn't modify the length.
            self.cache.append(&mut other.cache);
        } else {
            self.offset += others_offset;
            // other.offset < 0 means others pop something, nothing needs to insert.
            if others_offset > 0 {
                //means other push something.
                for (_, op) in other.cache.into_iter() {
                    let len = self.len().unwrap_or(0);
                    if self.offset > 0 {
                        self.cache.insert(len, op);
                    }
                    self.offset += 1;
                    self.len = Some(len + 1);
                }
            }
        }
    }
}
