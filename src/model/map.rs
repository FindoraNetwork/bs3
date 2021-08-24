use core::{fmt::Debug, mem};

use alloc::{collections::BTreeMap, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de> + Debug,
{
    value: BTreeMap<K, Operation<V>>,
}

impl<K, V> Default for Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn default() -> Self {
        Self {
            value: BTreeMap::new(),
        }
    }
}

impl<K, V> Model for Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn type_code(&self) -> u32 {
        3
    }

    #[cfg(feature = "cbor")]
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        let mut map = Vec::new();

        let value = mem::replace(&mut self.value, BTreeMap::new());

        for (k, v) in value.into_iter() {
            let key = serde_cbor::to_vec(&k)?;
            let value = v.to_bytes()?;
            map.push((key, value));
        }

        Ok(map)
    }
}
