use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{merkle::Merkle, model::Value, Result, SnapshotableStorage, Store, ValueStore};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<S, M, T> Tree for SnapshotableStorage<S, M, Value<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, _key: &Vec<u8>, _height: i64) -> Result<Vec<u8>> {
        let value = self.get()?;
        if let Some(val) = value {
            let bytes = serde_json::to_vec(val.as_ref())?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }
}
