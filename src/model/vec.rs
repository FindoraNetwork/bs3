//!
//! vec cache layer
//!
use core::{fmt::Debug, mem};

use crate::model::Model;
use crate::prelude::ToBytes;
use crate::{
    types::{Operation, OperationBytes},
    Result,
};
use alloc::{collections::BTreeMap, vec::Vec as AllocVec};

use super::ValueT;

/// define vec,inner value is btree
///     key : usize
#[derive(Debug, Clone)]
pub struct Vec<V>
where
    V: ValueT,
{
    pub(crate) value: BTreeMap<u64, Operation<V>>,
}

impl<V> Default for Vec<V>
where
    V: ValueT,
{
    fn default() -> Self {
        Self {
            value: BTreeMap::new(),
        }
    }
}

/// impl model
impl<V> Model for Vec<V>
where
    V: ValueT,
{
    /// Consume the data in the cache
    /// Also convert key to vec<u8>
    fn operations(&mut self) -> Result<AllocVec<(AllocVec<u8>, OperationBytes)>> {
        let mut map = AllocVec::new();

        let value = mem::replace(&mut self.value, BTreeMap::new());

        for (k, v) in value.into_iter() {
            let key = k.to_bytes()?;
            let value = v.to_bytes()?;
            map.push((key, value));
        }

        Ok(map)
    }

    /// define type 2
    fn type_code(&self) -> u32 {
        2
    }

    /// Merge two caches
    fn merge(&mut self, other: Self) {
        let mut value = other.value;
        self.value.append(&mut value);
    }
}
