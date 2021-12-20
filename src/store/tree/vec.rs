use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle, model::Vec as model_vec, Result, SnapshotableStorage, Store, VecStore,
};

use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<S, M, T> Tree for SnapshotableStorage<S, M, model_vec<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, key: &Vec<u8>, _height: i64) -> Result<Vec<u8>> {
        let key: u64 = serde_json::from_slice::<u64>(key)?;

        let value = self.get(key)?;
        if let Some(val) = value {
            let bytes = serde_json::to_vec(val.as_ref())?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }
}
