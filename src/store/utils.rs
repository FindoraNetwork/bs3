//!
//!

use alloc::vec::Vec as alloc_vec;
use core::fmt::Debug;

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use crate::{
    model::{DoubleKeyMap, Map, Value, Vec},
    Operation, Result, SnapshotableStorage, Store,
};
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub(crate) mod map_utils {
    use crate::merkle::Merkle;

    use super::*;

    pub fn get_inner_operation<S, M, K, V>(
        vss: &SnapshotableStorage<S, M, Map<K, V>>,
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
        M: Merkle,
    {
        let key_bytes = cbor_encode(key)?;
        let store_key = vss.storage_tuple_key(&key_bytes);
        let bytes = vss.store.get_ge2((&store_key.0, &store_key.1))?;
        // let store_key = vss.storage_key(&key_bytes);
        // let bytes = vss.store.get_ge(&*store_key)?;
        if let Some(bytes) = bytes {
            let value = StoreValue::from_bytes(&bytes)?;
            let operation = Operation::from_bytes(&value.operation)?;
            Ok(Some(operation))
        } else {
            Ok(None)
        }
    }

    pub fn get_inner_value<S, M, K, V>(
        vss: &SnapshotableStorage<S, M, Map<K, V>>,
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
        M: Merkle,
    {
        let operation = get_inner_operation(vss, key)?;
        if let Some(operation) = operation {
            match operation {
                Operation::Update(v) => Ok(Some(v)),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

pub(crate) mod doublekeymap_utils {
    use crate::merkle::Merkle;

    use super::*;

    pub fn get_inner_operation<S, M, K1, K2, V>(
        vss: &SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>,
        key: &(K1, K2),
    ) -> Result<Option<Operation<V>>>
    where
        K1: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
        K2: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
        V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
        S: Store,
        M: Merkle,
    {
        let key_bytes = cbor_encode(key)?;
        let store_key = vss.storage_tuple_key(&key_bytes);
        let bytes = vss.store.get_ge2((&store_key.0, &store_key.1))?;
        // let store_key = vss.storage_key(&key_bytes);
        // let bytes = vss.store.get_ge(&*store_key)?;
        if let Some(bytes) = bytes {
            let value = StoreValue::from_bytes(&bytes)?;
            let operation = Operation::from_bytes(&value.operation)?;
            Ok(Some(operation))
        } else {
            Ok(None)
        }
    }

    pub fn get_inner_value<S, M, K1, K2, V>(
        vss: &SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>,
        key: &(K1, K2),
    ) -> Result<Option<V>>
    where
        K1: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
        K2: Clone
            + PartialEq
            + Eq
            + Serialize
            + for<'de> Deserialize<'de>
            + Ord
            + PartialOrd
            + Debug,
        V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
        S: Store,
        M: Merkle,
    {
        let operation = get_inner_operation(vss, key)?;
        if let Some(operation) = operation {
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
    use crate::merkle::Merkle;

    use super::*;

    pub fn get_inner_value<S, M, T>(
        vss: &SnapshotableStorage<S, M, Vec<T>>,
        index: u64,
    ) -> Result<Option<T>>
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
        M: Merkle,
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

    pub fn get_inner_operation<S, M, T>(
        vss: &SnapshotableStorage<S, M, Vec<T>>,
        key: u64,
    ) -> Result<Option<Operation<T>>>
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
        M: Merkle,
    {
        let key_bytes = cbor_encode(key)?;
        let store_key = vss.storage_tuple_key(&key_bytes);
        let bytes = vss.store.get_ge2((&store_key.0, &store_key.1))?;
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
    use crate::merkle::Merkle;

    use super::*;

    pub fn storage_key<S, M, T>(
        vss: &SnapshotableStorage<S, M, Value<T>>,
    ) -> (alloc_vec<u8>, alloc_vec<u8>)
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
        M: Merkle,
    {
        let inner_key = alloc_vec::new();
        vss.storage_tuple_key(&inner_key)
    }

    pub fn get_inner_value<S, M, T>(vss: &SnapshotableStorage<S, M, Value<T>>) -> Result<Option<T>>
    where
        T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
        S: Store,
        M: Merkle,
    {
        let store_key = storage_key(vss);
        match vss.store.get_ge2((&store_key.0, &store_key.1))? {
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
