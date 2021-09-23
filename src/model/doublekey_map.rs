use core::{fmt::Debug, mem};

use alloc::vec::Vec;

#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

use crate::{OperationBytes, Result};
use crate::model::{Model, Map};

#[derive(Debug)]
pub struct DoubleKeyMap<K1, K2, V>
    where
        K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    pub(crate) value: Map<(K1,K2),V>,
}

impl<K1, K2, V> Default for DoubleKeyMap<K1, K2, V>
    where
        K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn default() -> Self {
        Self {
            value:Map::default(),
        }
    }
}

impl<K1, K2, V> Model for DoubleKeyMap<K1, K2, V>
    where
        K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
        V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{

    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        use crate::utils::cbor_encode;

        let mut map = Vec::new();

        let value = mem::replace(&mut self.value, Map::default());

        for (pair, v) in value.value.into_iter() {
            let key = cbor_encode(pair)?;
            let value = v.to_bytes()?;
            map.push((key, value));
        }

        Ok(map)
    }

    fn type_code(&self) -> u32 {
        4
    }

    fn merge(&mut self, other: Self) {
        let value = other.value;
        self.value.merge(value)
    }
}