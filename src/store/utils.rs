//!
//!

use alloc::vec::Vec as alloc_vec;
use core::fmt::Debug;

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use crate::{
    model::{DoubleKeyMap, Map, Value, ValueType, Vec, INDEX_VEC_LEN},
    Operation, Result, SnapshotableStorage, Store,
};
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub fn get_greatest<S: Store, V: ValueType>(
    store: &S,
    from: &[u8],
    to: &[u8],
) -> Result<Option<V>> {
    let bytes = store.get_ge2((from, to))?;
    if let Some(bytes) = bytes {
        let value = StoreValue::from_bytes(&bytes)?;
        let operation = Operation::from_bytes(&value.operation)?;
        Ok(match operation {
            Operation::Update(t) => t,
            Operation::Delete => None,
        })
    } else {
        Ok(None)
    }
}

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

pub(crate) mod value_utils {
    use crate::merkle::Merkle;

    use super::*;

    pub fn storage_key<S, M, T>(
        vss: &SnapshotableStorage<S, M, Value<T>>,
    ) -> (alloc_vec<u8>, alloc_vec<u8>)
    where
        T: ValueType,
        S: Store,
        M: Merkle,
    {
        let inner_key = alloc_vec::new();
        vss.storage_tuple_key(&inner_key)
    }

    pub fn get_inner_value<S, M, T>(vss: &SnapshotableStorage<S, M, Value<T>>) -> Result<Option<T>>
    where
        T: ValueType,
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
