use core::{fmt::Debug, mem};

use alloc::{collections::BTreeMap, vec::Vec};

#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

use crate::{Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Map<K, V>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    pub(crate) value: BTreeMap<K, Operation<V>>,
}

impl<K, V> Default for Map<K, V>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn default() -> Self {
        Self {
            value: BTreeMap::new(),
        }
    }
}

impl<K, V> Model for Map<K, V>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn type_code(&self) -> u32 {
        3
    }

    #[cfg(feature = "cbor")]
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        use crate::utils::cbor_encode;

        let mut map = Vec::new();

        let value = mem::replace(&mut self.value, BTreeMap::new());

        for (k, v) in value.into_iter() {
            let key = cbor_encode(k)?;
            let value = v.to_bytes()?;
            map.push((key, value));
        }

        Ok(map)
    }

    fn merge(&mut self, other: Self) {
        let mut value = other.value;
        self.value.append(&mut value);
    }
}
