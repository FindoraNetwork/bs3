use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle, model::DoubleKeyMap, DoubleKeyMapStore, Result, SnapshotableStorage, Store,
};

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
    fn tree_get(&self, key: &[u8]) -> Result<Vec<u8>> {
        let key = serde_json::from_slice::<(K1, K2)>(key)?;

        let value = self.get(&key.0, &key.1)?;
        if let Some(val) = value {
            let bytes = serde_json::to_vec(val.as_ref())?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }
}
