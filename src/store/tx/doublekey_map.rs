use crate::merkle::Merkle;
use crate::model::DoubleKeyMap;
use crate::{Cow, DoubleKeyMapStore, Operation, Result, Store, Transaction};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};
use crate::store::utils::doublekeymap_utils;

impl<'a, S, M, K1, K2, V> DoubleKeyMapStore<K1, K2, V> for Transaction<'a, S, M, DoubleKeyMap<K1, K2, V>>
where
    K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
    M: Merkle,
{
    fn get(&self, key1: &K1, key2: &K2) -> Result<Option<Cow<'_, V>>> {
        let key = &(key1.clone(), key2.clone());
        let self_value = self.value.value.value.get(key);

        Ok(match self_value {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => {
                let lower_value = self.store.value.value.value.get(key);
                match lower_value {
                    Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
                    Some(Operation::Delete) => None,
                    None => None
                }
            }
        })
    }

    fn get_mut(&mut self, key1: &K1, key2: &K2) -> Result<Option<&mut V>> {
        let key = &(key1.clone(), key2.clone());
        if let Some(Operation::Delete) = self.value.value.value.get(key) {
            return Ok(None);
        }

        if !self.value.value.value.contains_key(key) {
            if let Some(operation) = doublekeymap_utils::get_inner_operation(self.store, key)? {
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
        let key = (key1, key2);
        let operation = Operation::Update(value.clone());
        let mut pre_val = None;
        if let Some(operation) = self.value.value.value.get_mut(&key) {
            match operation {
                Operation::Update(v) => {
                    pre_val = Some(v.clone());
                }
                Operation::Delete => {}
            }
        }
        self.value.value.value.insert(key, operation);
        Ok(pre_val)
    }

    fn remove(&mut self, key1: &K1, key2: &K2) -> Result<Option<V>> {
        let key = &(key1.clone(), key2.clone());
        let res = if let Some(op) = self.value.value.value.remove(key) {
            match op {
                Operation::Update(v) => Some(v),
                Operation::Delete => None,
            }
        } else {
            if let Some(v) = self.store.get(key1, key2)? {
                Some(v.clone())
            } else {
                None
            }
        };

        self.value.value.value.insert(key.clone(), Operation::Delete);

        Ok(res)
    }
}
