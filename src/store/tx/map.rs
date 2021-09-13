use super::utils::map_utils;
use crate::model::Map;
use crate::{Cow, MapStore, Operation, Store, Transaction};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<'a, S, K, V> MapStore<K, V> for Transaction<'a, S, Map<K, V>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    fn get(&self, key: &K) -> crate::Result<Option<Cow<'_, V>>> {
        return if let Some(operation) = self.value.value.get(key) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            if let Some(v) = map_utils::get_inner_value(self.store, key)? {
                Ok(Some(Cow::Owned(v)))
            } else {
                Ok(None)
            }
        };
    }

    fn get_mut(&mut self, key: K) -> crate::Result<Option<&mut V>> {
        if let Some(Operation::Delete) = self.value.value.get(&key) {
            return Ok(None);
        }

        if !self.value.value.contains_key(&key) {
            if let Some(operation) = map_utils::get_inner_operation(self.store, &key)? {
                self.value.value.insert(key.clone(), operation);
            } else {
                return Ok(None);
            }
        }

        if let Operation::Update(value) = self.value.value.get_mut(&key).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key: K, value: V) -> crate::Result<Option<V>> {
        let operation = Operation::Update(value.clone());
        self.value.value.insert(key.clone(), operation);
        map_utils::get_inner_value(self.store, &key)
    }

    fn remove(&mut self, key: K) -> crate::Result<Option<V>> {
        self.value.value.remove(&key);
        map_utils::get_inner_value(self.store, &key)
    }
}
