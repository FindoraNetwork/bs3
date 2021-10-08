use crate::model::DoubleKeyMap;
use crate::{Cow, DoubleKeyMapStore, Operation, Store, Transaction};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<'a, S, K1, K2, V> DoubleKeyMapStore<K1, K2, V> for Transaction<'a, S, DoubleKeyMap<K1, K2, V>>
where
    K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    fn get(&self, key1: &K1, key2: &K2) -> crate::Result<Option<Cow<'_, V>>> {
        let key = &(key1.clone(), key2.clone());
        return if let Some(operation) = self.value.value.value.get(key) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        };
    }

    fn get_mut(&mut self, key1: &K1, key2: &K2) -> crate::Result<Option<&mut V>> {
        let key = &(key1.clone(), key2.clone());
        if let Operation::Update(value) = self.value.value.value.get_mut(key).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> crate::Result<Option<V>> {
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

    fn remove(&mut self, key1: &K1, key2: &K2) -> crate::Result<Option<V>> {
        let key = &(key1.clone(), key2.clone());
        return if let Some(op) = self.value.value.value.remove(key) {
            match op {
                Operation::Update(v) => Ok(Some(v)),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        };
    }
}
