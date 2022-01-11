use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle,
    model::{Value, ValueType},
    Result, SnapshotableStorage, Store, ValueStore,
};

impl<S, M, T> Tree for SnapshotableStorage<S, M, Value<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, _key: &[u8]) -> Result<Vec<u8>> {
        let value = self.get()?;
        if let Some(val) = value {
            let bytes = serde_json::to_vec(val.as_ref())?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }
}
