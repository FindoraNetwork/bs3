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
            Ok(None)
        };
    }

    fn get_mut(&mut self, key: &K) -> crate::Result<Option<&mut V>> {
        if let Operation::Update(value) = self.value.value.get_mut(key).unwrap() {
            Ok(Some(value))
        } else {
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
        return if let Some(op) = self.value.value.remove(key) {
            match op {
                Operation::Update(v) => Ok(Some(v)),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        };
    }
}
