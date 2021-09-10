use alloc::vec::Vec as alloc_vec;
use core::fmt::Debug;

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use crate::{
    model::{Map, Value, Vec},
    Operation, Result, SnapshotableStorage, Store,
};
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub(crate) mod map_utils {
    use super::*;

    pub fn get_inner_operation<S, K, V>(
        vss: &SnapshotableStorage<S, Map<K, V>>,
        key: &K,
    ) -> Result<Option<Operation<V>>>
    where
        K: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
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

    pub fn get_inner_value<S, K, V>(
        vss: &SnapshotableStorage<S, Map<K, V>>,
        key: &K,
    ) -> Result<Option<V>>
    where
        K: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
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
}

pub(crate) mod vec_utils {
    use super::*;

    pub fn get_inner_value<S, T>(
        vss: &SnapshotableStorage<S, Vec<T>>,
        index: usize,
    ) -> Result<Option<T>>
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
    {
        let operation = get_inner_operation(vss, index)?;
        if let Some(operation) = operation {
            match operation {
                Operation::Update(v) => Ok(Some(v)),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_inner_operation<S, T>(
        vss: &SnapshotableStorage<S, Vec<T>>,
        key: usize,
    ) -> Result<Option<Operation<T>>>
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
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
}

pub(crate) mod value_utils {
    use super::*;

    pub fn storage_key<S, T>(vss: &SnapshotableStorage<S, Value<T>>) -> alloc_vec<u8>
    where
        T: Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
    {
        let inner_key = alloc_vec::new();
        vss.storage_key(&inner_key)
    }

    pub fn get_inner_value<S, T>(vss: &SnapshotableStorage<S, Value<T>>) -> Result<Option<T>>
    where
        T: Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
    {
        let key = storage_key(vss);
        match vss.store.get_ge(&key)? {
            Some(bytes) => {
                let value = StoreValue::from_bytes(&bytes)?;
                let operation = Operation::from_bytes(&value.operation)?;
                match operation {
                    Operation::Update(v) => Ok(Some(v)),
                    Operation::Delete => Ok(None),
                }
            }
            None => Ok(None),
        }
    }
}