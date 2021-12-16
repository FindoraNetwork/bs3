use crate::prelude::Tree;
use alloc::vec::Vec;

use crate::{
    merkle::Merkle, model::Vec as model_vec, Operation, Result, SnapshotableStorage, Store,
};

use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<S, M, T> Tree for SnapshotableStorage<S, M, model_vec<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
    M: Merkle,
{
    fn tree_get(&self, key: &Vec<u8>, height: i64) -> Result<Option<Vec<u8>>> {
        let (k1, k2) = self.storage_tuple_key_with_height(key, height);
        let bytes = self.store.get_ge2((&k1, &k2))?;
        if let Some(bytes) = bytes {
            let value = StoreValue::from_bytes(&bytes)?;
            let operation = Operation::<T>::from_bytes(&value.operation)?;
            match operation {
                Operation::Update(v) => {
                    log::debug!("tree get value:{:?}", v);
                    let bytes = cbor_encode(v)?;
                    Ok(Some(bytes))
                }
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
