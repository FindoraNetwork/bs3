use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{merkle::Merkle, model::DoubleKeyMap, Operation, Result, SnapshotableStorage, Store};

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<S, M, K1, K2, V> Tree for SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>
where
    K1: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    K2: Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Debug,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, key: &Vec<u8>, height: i64) -> Result<Vec<u8>> {
        let key: K1 = serde_json::from_slice::<K1>(key)?;

        let key_bytes = cbor_encode(key)?;

        let (k1, k2) = self.storage_tuple_key_with_height(&key_bytes, height);
        let bytes = self.store.get_ge2((&k1, &k2))?;
        if let Some(bytes) = bytes {
            let value = StoreValue::from_bytes(&bytes)?;
            let operation = Operation::<V>::from_bytes(&value.operation)?;
            match operation {
                Operation::Update(v) => {
                    log::debug!("tree get value:{:?}", v);
                    let bytes = cbor_encode(v)?;
                    Ok(bytes)
                }
                Operation::Delete => Ok(Vec::new()),
            }
        } else {
            Ok(Vec::new())
        }
    }
}
