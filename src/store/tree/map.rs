use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle,
    model::{KeyType, Map, ValueType},
    MapStore, Result, SnapshotableStorage, Store,
};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<S, M, K, V> Tree for SnapshotableStorage<S, M, Map<K, V>>
where
    K: KeyType,
    V: ValueType,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, key: &Vec<u8>) -> Result<Vec<u8>> {
        let key: K = serde_json::from_slice::<K>(key)?;

        let value = self.get(&key)?;
        if let Some(val) = value {
            let bytes = serde_json::to_vec(val.as_ref())?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }
}
