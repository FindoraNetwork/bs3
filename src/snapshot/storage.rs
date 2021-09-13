use alloc::{string::String, vec::Vec};

use crate::{backend::Store, model::Model, snapshot::StoreValue, Error, Result};

use super::{utils, value::StoreType, FromStoreBytes, StoreHeight, ToStoreBytes, Transaction};

/// Snapshotable Storage
pub struct SnapshotableStorage<S, M>
where
    S: Store,
    M: Model,
{
    pub(crate) store: S,
    pub(crate) height: i64,
    pub(crate) value: M,
    pub(crate) namespace: String,
}

/// Methods for create storage.
impl<S, M> SnapshotableStorage<S, M>
where
    S: Store,
    M: Model,
{
    /// Create a `SnapshotableStorage` from store with empty namespace.
    pub fn new(value: M, store: S) -> Result<Self> {
        Self::new_with_name(value, String::new(), store)
    }

    /// Create a `SnapshotableStorage` from store.
    pub fn new_with_name(value: M, name: String, store: S) -> Result<Self> {
        let mut s = Self {
            store,
            height: 0,
            namespace: name,
            value,
        };

        if !s.init_or_load()? {
            // load height.

            log::debug!(
                "Load snapshotable store with namespace {}, at height: {}",
                s.namespace,
                s.height
            );
            s.height = s.read_height()?;
        }

        Ok(s)
    }

    /// Create a `SnapshotableStorage` from store for point height with empty namespace.
    ///
    /// If height is 0, equal to init a store.
    pub fn new_with_height(value: M, height: i64, store: S) -> Result<Self> {
        Self::new_with_height_namespace(value, height, String::new(), store)
    }

    /// Create a `SnapshotableStorage` from store for point height.
    ///
    /// If height is 0, equal to init a store.
    pub fn new_with_height_namespace(
        value: M,
        height: i64,
        namespace: String,
        store: S,
    ) -> Result<Self> {
        let mut s = Self {
            store: store,
            height,
            value,
            namespace,
        };

        if height == 0 {
            s.init()?;
        } else {
            s.rollback(height)?;
        };

        Ok(s)
    }

    /// Init a new store.
    fn init(&mut self) -> Result<()> {
        let key = utils::type_key(&self.namespace);

        log::debug!(
            "Inital snapshotable store with namespace: {}",
            self.namespace
        );

        let store_type = StoreType {
            ty: self.value.type_code(),
        };
        let bytes = store_type.to_bytes()?;
        self.store.insert(key, bytes)?;
        self.write_height(0, None)?;
        Ok(())
    }

    /// Init or load a new store.
    fn init_or_load(&mut self) -> Result<bool> {
        let key = utils::type_key(&self.namespace);
        if let Some(bytes) = self.store.get_ge(&key)? {
            let store_type = StoreType::from_bytes(&bytes)?;
            match store_type.ty == self.value.type_code() {
                true => Ok(false),
                false => Err(Error::TypeMissMatch),
            }
        } else {
            self.init()?;
            Ok(true)
        }
    }

    pub fn store(&self) -> &S {
        &self.store
    }
}

/// Methods for snapshot.
impl<S, M> SnapshotableStorage<S, M>
where
    S: Store,
    M: Model,
{
    /// Read current height in store.
    fn read_height(&self) -> Result<i64> {
        let key = utils::current_height_key(&self.namespace);

        if let Some(bytes) = self.store.get_ge(&key)? {
            let store_height = StoreHeight::from_bytes(&bytes)?;
            Ok(store_height.height)
        } else {
            Ok(0)
        }
    }

    /// Force to write height in store.
    fn write_height(
        &mut self,
        target_height: i64,
        pre_commit: Option<Vec<(Vec<u8>, Vec<u8>)>>,
    ) -> Result<()> {
        log::debug!("Begin sync snapshot success in height: {}", self.height + 1);
        self.height = target_height;

        let mut operations = if let Some(ops) = pre_commit {
            ops
        } else {
            Vec::new()
        };

        let store_height = StoreHeight {
            height: self.height,
        };
        let height_key_bytes = utils::current_height_key(&self.namespace);
        let height_value_bytes = store_height.to_bytes()?;
        operations.push((height_key_bytes, height_value_bytes));
        self.store.execute(operations)?;

        Ok(())
    }

    /// rollback to point height, target_height must less than current height.
    pub fn rollback(&mut self, target_height: i64) -> Result<()> {
        if target_height > self.height {
            log::error!("Target height must less than current height");
            Err(Error::HeightError)
        } else {
            self.write_height(target_height, None)
        }
    }

    pub(crate) fn storage_key(&self, key: &Vec<u8>) -> Vec<u8> {
        utils::storage_key(&self.namespace, &key, self.height)
    }

    pub(crate) fn storage_tuple_key(&self, key: &Vec<u8>) -> (Vec<u8>,Vec<u8>){
        (
            utils::storage_key(&self.namespace, &key, 0),
            utils::storage_key(&self.namespace, &key, self.height)
        )
    }

    /// Commit this snapshot.
    pub fn commit(&mut self) -> Result<i64> {
        let mut operations = Vec::new();

        // exchange cache.
        // let mut cache = mem::replace(&mut self.value, M::default());

        log::debug!("Snapshot Cache: {:#?}", self.value);

        for (k, v) in self.value.operations()? {
            let key_bytes = self.storage_key(&k);
            let store_value = StoreValue { operation: v };
            operations.push((key_bytes, store_value.to_bytes()?));
        }

        // incr current height
        self.write_height(self.height + 1, Some(operations))?;

        log::debug!("Sync snapshot success in height: {}", self.height);

        Ok(self.height)
    }
}

/// Methods for transaction
impl<S, M> SnapshotableStorage<S, M>
where
    S: Store,
    M: Model,
{
    /// Generate transaction for this Bs3 db.
    pub fn transaction(&self) -> Transaction<'_, S, M> {
        Transaction::new(self)
    }

    /// Consume transaction to apply.
    pub fn execute(&mut self, tx: Transaction<'_, S, M>) {
        log::debug!("Transaction Cache: {:#?}", tx.value);
        self.value.merge(tx.value)
    }
}

// /// Methods for internal helper
// impl<S, D> SnapshotableStorage<S, D>
// where
//     D: Digest,
//     S: Store,
// {
//
//    fn raw_insert(&mut self, key: &Output<D>, value: OperationOwned) -> Result<Option<Vec<u8>>> {
//         Ok(match self.cache.insert(key.clone(), value) {
//             Some(OperationOwned::Update(v)) => Some(v),
//             Some(OperationOwned::Delete) => None,
//             None => match self.raw_get_lt(key, self.height)? {
//                 Some(bytes) => {
//                     let r = StoreValue::from_bytes(&bytes)?;
//                     match r.operation {
//                         Operation::Update(v) => Some(Vec::from(v)),
//                         Operation::Delete => None,
//                     }
//                 }
//                 None => None,
//             },
//         })
//     }
// }
//
// impl<S, D> Tree<D> for SnapshotableStorage<S, D>
// where
//     D: Digest,
//     S: Store,
// {
//     fn get(&self, key: &Output<D>) -> Result<Option<BytesRef<'_>>> {
//         let cache_result = self.cache.get(key);
//         match cache_result {
//             Some(OperationOwned::Update(v)) => Ok(Some(BytesRef::Borrow(v.as_slice()))),
//             Some(OperationOwned::Delete) => Ok(None),
//             None => match self.raw_get_lt(key, self.height)? {
//                 // I can't write result to cache.
//                 // If want cache to speed up, in lower `Store`.
//                 Some(bytes) => {
//                     let r = StoreValue::from_bytes(&bytes)?;
//                     match r.operation {
//                         // this need optine, beacuse this cause copy
//                         Operation::Update(v) => Ok(Some(BytesRef::Owned(Vec::from(v)))),
//                         Operation::Delete => Ok(None),
//                     }
//                 }
//                 None => Ok(None),
//             },
//         }
//     }
// }
//
// impl<S, D> TreeMut<D> for SnapshotableStorage<S, D>
// where
//     D: Digest,
//     S: Store,
// {
//     fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
//         if let Some(OperationOwned::Delete) = self.cache.get(key) {
//             return Ok(None);
//         }
//         if self.cache.contains_key(key) {
//             match self.raw_get_lt(key, self.height)? {
//                 Some(bytes) => {
//                     let r = StoreValue::from_bytes(&bytes)?;
//                     // this assign to prevent #[warn(mutable_borrow_reservation_conflict)]
//                     let operation_owned = r.operation.to_operation_owned();
//                     self.cache.insert(key.clone(), operation_owned);
//                 }
//                 None => return Ok(None),
//             }
//         }
//
//         // I'm sure this value exists.
//         if let OperationOwned::Update(value) = self.cache.get_mut(key).unwrap() {
//             Ok(Some(value))
//         } else {
//             Ok(None)
//         }
//     }
//
//     fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
//         self.raw_insert(&key, OperationOwned::Update(value))
//     }
//
//     fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
//         self.raw_insert(key, OperationOwned::Delete)
//     }
// }
