//!
//! vec cache layer
//!
use core::{fmt::Debug, mem};

use crate::model::Model;
use crate::{Operation, OperationBytes};
use alloc::{collections::BTreeMap, vec::Vec as alloc_vec};
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

/// define vec,inner value is btree
///     key : usize
#[derive(Debug)]
pub struct Vec<V>
where
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    pub(crate) value: BTreeMap<u64, Operation<V>>,
}

impl<V> Default for Vec<V>
where
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
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
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    /// Consume the data in the cache
    /// Also convert key to vec<u8>
    fn operations(&mut self) -> crate::Result<alloc_vec<(alloc_vec<u8>, OperationBytes)>> {
        use crate::utils::cbor_encode;

        let mut map = alloc_vec::new();

        let value = mem::replace(&mut self.value, BTreeMap::new());

        for (k, v) in value.into_iter() {
            let key = cbor_encode(k)?;
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
