use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle,
    model::{ValueType, Vec as ModelVec},
    Result, SnapshotableStorage, Store, VecStore,
};

impl<S, M, T> Tree for SnapshotableStorage<S, M, ModelVec<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, key: &[u8]) -> Result<Vec<u8>> {
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
