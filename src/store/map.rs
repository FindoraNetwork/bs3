use core::{borrow::Borrow, fmt::Debug};

use crate::{
    merkle::Merkle,
    model::{KeyType, Map, ValueType},
    Cow, Operation, Result, SnapshotableStorage, Store,
};

use super::utils::map_utils;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

/// Defining the basic behavior of the map application layer
pub trait MapStore<K, V>
where
    K: KeyType,
    V: ValueType,
{
    fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>>;

    fn get_mut(&mut self, key: &K) -> Result<Option<&mut V>>;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>>;

    fn remove(&mut self, key: &K) -> Result<Option<V>>;
}

/// Implementing the middle and cache layers is the behavior of map
impl<S, M, K, V> MapStore<K, V> for SnapshotableStorage<S, M, Map<K, V>>
where
    K: KeyType,
    V: ValueType,
    S: Store,
    M: Merkle,
{
    fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>> {
        return if let Some(operation) = self.value.value.get(key) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            if let Some(v) = map_utils::get_inner_value(self, key)? {
                Ok(Some(Cow::Owned(v)))
            } else {
                Ok(None)
            }
        };
    }

    fn get_mut(&mut self, key: &K) -> Result<Option<&mut V>> {
        if let Some(Operation::Delete) = self.value.value.get(key) {
            return Ok(None);
        }

        if !self.value.value.contains_key(key) {
            if let Some(operation) = map_utils::get_inner_operation(self, key)? {
                self.value.value.insert(key.clone(), operation);
            } else {
                return Ok(None);
            }
        }

        if let Some(Operation::Update(value)) = self.value.value.get_mut(key) {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>> {
        let operation = Operation::Update(value.clone());
        self.value.value.insert(key.clone(), operation);
        map_utils::get_inner_value(self, &key)
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>> {
        let res = if let Some(op) = self.value.value.remove(key) {
            match op {
                Operation::Update(v) => Some(v),
                Operation::Delete => None,
            }
        } else {
            if let Some(v) = map_utils::get_inner_value(self, key)? {
                Some(v)
            } else {
                None
            }
        };

        self.value.value.insert(key.clone(), Operation::Delete);

        Ok(res)
    }
}
