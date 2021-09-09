use core::fmt::Debug;

use crate::{model::Map, Cow, Operation, Result, SnapshotableStorage, Store};

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use core::option::Option::Some;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub trait MapStore<K, V>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
{
    fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>>;

    fn get_mut(&mut self, key: K) -> Result<Option<&mut V>>;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>>;

    fn remove(&mut self, key: K) -> Result<Option<V>>;
}

impl<S, K, V> MapStore<K, V> for SnapshotableStorage<S, Map<K, V>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    fn get(&self, key: &K) -> Result<Option<Cow<'_, V>>> {
        return if let Some(operation) = self.value.value.get(key) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            if let Some(v) = get_inner_value(self, key)? {
                Ok(Some(Cow::Owned(v)))
            } else {
                Ok(None)
            }
        };
    }

    fn get_mut(&mut self, key: K) -> Result<Option<&mut V>> {
        if let Some(Operation::Delete) = self.value.value.get(&key) {
            return Ok(None);
        }

        if !self.value.value.contains_key(&key) {
            if let Some(operation) = get_mut_inner_operation(self, &key)? {
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

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>> {
        let operation = Operation::Update(value.clone());
        self.value.value.insert(key.clone(), operation);
        get_mut_inner_value(self, &key)
    }

    fn remove(&mut self, key: K) -> Result<Option<V>> {
        self.value.value.remove(&key);
        get_mut_inner_value(self, &key)
    }
}

fn get_mut_inner_value<S, K, V>(
    vss: &mut SnapshotableStorage<S, Map<K, V>>,
    key: &K,
) -> Result<Option<V>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    let operation = get_mut_inner_operation(vss, key)?;
    if let Some(operation) = operation {
        match operation {
            Operation::Update(v) => Ok(Some(v)),
            Operation::Delete => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn get_mut_inner_operation<S, K, V>(
    vss: &mut SnapshotableStorage<S, Map<K, V>>,
    key: &K,
) -> Result<Option<Operation<V>>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    let key_bytes = cbor_encode(key)?;
    let store_key = vss.storage_key(&key_bytes);
    let bytes = vss.store.get_ge(&*store_key)?;
    if let Some(bytes) = bytes {
        let value = StoreValue::from_bytes(&bytes)?;
        let operation = Operation::from_bytes(&value.operation)?;
        Ok(Some(operation))
    } else {
        Ok(None)
    }
}

fn get_inner_value<S, K, V>(vss: &SnapshotableStorage<S, Map<K, V>>, key: &K) -> Result<Option<V>>
where
    K: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
{
    let key_bytes = cbor_encode(key)?;
    let store_key = vss.storage_key(&key_bytes);
    let bytes = vss.store.get_ge(&*store_key)?;
    if let Some(bytes) = bytes {
        let value = StoreValue::from_bytes(&bytes)?;
        let operation = Operation::from_bytes(&value.operation)?;
        match operation {
            Operation::Update(v) => Ok(Some(v)),
            Operation::Delete => Ok(None),
        }
    } else {
        Ok(None)
    }
}
