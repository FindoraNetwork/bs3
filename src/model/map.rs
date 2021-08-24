use core::fmt::Debug;

use alloc::{collections::BTreeMap, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{Cow, Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    value: BTreeMap<K, Operation<V>>,
}

impl<K, V> Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>> {
        Ok(None)
    }

    pub fn get_mut(&self, key: &K) -> Result<Option<&mut V>> {
        Ok(None)
    }
}

impl<K, V> Model for Map<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de>,
    V: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn operations(&self) -> Result<BTreeMap<Vec<u8>, OperationBytes>> {
        let mut map = BTreeMap::new();

        for (k, v) in self.value.iter() {
            let key = serde_cbor::to_vec(k)?;
            let value = v.to_bytes()?;
            map.insert(key, value);
        }

        Ok(map)
    }
}
