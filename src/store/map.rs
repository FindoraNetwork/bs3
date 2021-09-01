use core::fmt::Debug;

use crate::{model::Map, Cow, Result, SnapshotableStorage, Store};

#[cfg(feature = "cbor")]
use minicbor::{Decode as Deserialize, Encode as Serialize};

pub trait MapStore<K, V>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>>;

    fn get_mut(&mut self, key: &K) -> Result<Option<&mut V>>;

    fn insert(&mut self, key: &K, value: V) -> Result<Option<V>>;

    fn remove(&mut self, key: K) -> Result<Option<V>>;
}

impl<S, K, V> MapStore<K, V> for SnapshotableStorage<S, Map<K, V>>
where
    K: PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    fn get(&self, _key: &K) -> Result<Option<Cow<'_, V>>> {
        Ok(None)
    }

    fn get_mut(&mut self, _key: &K) -> Result<Option<&mut V>> {
        Ok(None)
    }

    fn insert(&mut self, _key: &K, _value: V) -> Result<Option<V>> {
        Ok(None)
    }

    fn remove(&mut self, _key: K) -> Result<Option<V>> {
        Ok(None)
    }
}
