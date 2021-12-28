//!
//! map cache layer

use core::{fmt::Debug, mem};

use alloc::{collections::BTreeMap, vec::Vec};

use crate::{Operation, OperationBytes, Result};

use super::{KeyT, Model, ValueT};

/// define cache map
/// use BTree
///     key:K
///     value:Operation<V>
#[derive(Debug, Clone)]
pub struct Map<K, V>
where
    K: KeyT,
    V: ValueT,
{
    pub(crate) value: BTreeMap<K, Operation<V>>,
}

impl<K, V> Default for Map<K, V>
where
    K: KeyT,
    V: ValueT,
{
    fn default() -> Self {
        Self {
            value: BTreeMap::new(),
        }
    }
}

/// impl Model
impl<K, V> Model for Map<K, V>
where
    K: KeyT,
    V: ValueT,
{
    ///define type 3
    fn type_code(&self) -> u32 {
        3
    }

    /// Consume the data in the cache
    /// Also convert key to vec<u8>
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        let mut map = Vec::new();

        let value = mem::replace(&mut self.value, BTreeMap::new());

        for (k, v) in value.into_iter() {
            let key = k.to_bytes()?;
            let value = v.to_bytes()?;
            map.push((key, value));
        }

        Ok(map)
    }

    /// Merge two caches
    fn merge(&mut self, other: Self) {
        let mut value = other.value;
        self.value.append(&mut value);
    }
}
