use crate::merkle::Merkle;
use crate::model::Map;
use crate::{Cow, MapStore, Operation, Store, Transaction};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<'a, S, M, K, V> MapStore<K, V> for Transaction<'a, S, M, Map<K, V>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
    M: Merkle,
{
    fn get(&self, key: &K) -> crate::Result<Option<Cow<'_, V>>> {
        let self_value = self.value.value.get(key);
        Ok(match self_value {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => {
                let lower_value = self.store.value.value.get(key);
                match lower_value {
                    Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
                    Some(Operation::Delete) => None,
                    None => {
                        let store_inner_value = self.store.get(key)?;
                        match store_inner_value {
                            None => None,
                            Some(v) => Some(v),
                        }
                    }
                }
            }
        })
    }

    fn get_mut(&mut self, key: &K) -> crate::Result<Option<&mut V>> {
        if let Some(Operation::Delete) = self.value.value.get(key) {
            return Ok(None);
        }

        if !self.value.value.contains_key(key) {
            let lower_value = self.store.get(key)?;
            if let Some(v) = lower_value {
                let value = v.clone();
                self.value
                    .value
                    .insert(key.clone(), Operation::Update(value));
            } else {
                return Ok(None);
            }
        }

        // I'm ensure here has value.
        if let Some(Operation::Update(v)) = self.value.value.get_mut(key) {
            Ok(Some(v))
        } else {
            // So this branch will never enter.
            Ok(None)
        }
    }

    fn insert(&mut self, key: K, value: V) -> crate::Result<Option<V>> {
        let operation = Operation::Update(value.clone());
        let mut pre_val = None;
        if let Some(operation) = self.value.value.get_mut(&key) {
            match operation {
                Operation::Update(v) => {
                    pre_val = Some(v.clone());
                }
                Operation::Delete => {}
            }
        }
        self.value.value.insert(key, operation);
        Ok(pre_val)
    }

    fn remove(&mut self, key: &K) -> crate::Result<Option<V>> {
        let res = if let Some(op) = self.value.value.remove(key) {
            match op {
                Operation::Update(v) => Some(v),
                Operation::Delete => None,
            }
        } else {
            if let Some(v) = self.store.get(key)? {
                Some(v.clone())
            } else {
                None
            }
        };

        self.value.value.insert(key.clone(), Operation::Delete);

        Ok(res)
    }
}
