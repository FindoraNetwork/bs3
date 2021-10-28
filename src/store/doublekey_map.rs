use core::fmt::Debug;

use crate::{
    merkle::Merkle, model::DoubleKeyMap, Cow, Operation, Result, SnapshotableStorage, Store,
};

use super::utils::doublekeymap_utils;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub trait DoubleKeyMapStore<K1, K2, V>
where
    K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn get(&self, key1: &K1, key2: &K2) -> Result<Option<Cow<'_, V>>>;

    fn get_mut(&mut self, key1: &K1, key2: &K2) -> Result<Option<&mut V>>;

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> Result<Option<V>>;

    fn remove(&mut self, key1: &K1, key2: &K2) -> Result<Option<V>>;
}

impl<S, M, K1, K2, V> DoubleKeyMapStore<K1, K2, V>
    for SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>
where
    K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
    M: Merkle,
{
    fn get(&self, key1: &K1, key2: &K2) -> Result<Option<Cow<'_, V>>> {
        let key = &(key1.clone(), key2.clone());
        return if let Some(operation) = self.value.value.value.get(key) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            if let Some(v) = doublekeymap_utils::get_inner_value(self, key)? {
                Ok(Some(Cow::Owned(v)))
            } else {
                Ok(None)
            }
        };
    }

    fn get_mut(&mut self, key1: &K1, key2: &K2) -> Result<Option<&mut V>> {
        let key = &(key1.clone(), key2.clone());
        if let Some(Operation::Delete) = self.value.value.value.get(key) {
            return Ok(None);
        }

        if !self.value.value.value.contains_key(key) {
            if let Some(operation) = doublekeymap_utils::get_inner_operation(self, key)? {
                self.value.value.value.insert(key.clone(), operation);
            } else {
                return Ok(None);
            }
        }

        if let Some(Operation::Update(value)) = self.value.value.value.get_mut(key) {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> Result<Option<V>> {
        let operation = Operation::Update(value.clone());
        let key = (key1, key2);
        self.value.value.value.insert(key.clone(), operation);
        doublekeymap_utils::get_inner_value(self, &key)
    }

    fn remove(&mut self, key1: &K1, key2: &K2) -> Result<Option<V>> {
        let key = &(key1.clone(), key2.clone());
        let res = if let Some(op) = self.value.value.value.remove(key) {
            match op {
                Operation::Update(v) => Some(v),
                Operation::Delete => None,
            }
        } else {
            if let Some(v) = self.get(key1, key2)? {
                Some(v.clone())
            } else {
                None
            }
        };

        self.value
            .value
            .value
            .insert(key.clone(), Operation::Delete);

        Ok(res)
    }
}
