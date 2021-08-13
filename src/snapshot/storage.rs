use core::mem;

use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::{Error, Result, backend::Store};

use super::{Operation, StoreHeight, StoreValue, ToStoreBytes, operation::OperationOwned, utils};


/// Snapshotable Storage
pub struct SnapshotableStorage<'a, D: Digest> {
    store: &'a dyn Store,
    height: u64,
    cache: BTreeMap<Output<D>, OperationOwned>,
    namespace: &'a str,
}

/// Methods for create storage.
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    pub fn new(store: &'a impl Store) -> Self {
        Self::new_with_name(store, "")
    }

    pub fn new_with_name(store: &'a impl Store, name: &'a str) -> Self {
        Self {
            store: store as &'a dyn Store,
            height: 0,
            cache: BTreeMap::new(),
            namespace: name,
        }
    }

    pub fn new_with_height(store: &'a impl Store, height: u64) -> Result<Self> {
        let mut s = Self::new(store);
        s.rollback(height)?;
        Ok(s)
    }

    pub fn new_with_height_namespace(
        store: &'a impl Store,
        height: u64,
        namespace: &'a str,
    ) -> Result<Self> {
        let mut s = Self {
            store: store as &'a dyn Store,
            height,
            cache: BTreeMap::new(),
            namespace,
        };
        s.rollback(height)?;
        Ok(s)
    }
}

/// Methods for snapshot.
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    /// rollback to point height, target_height must less than current height.
    pub fn rollback(&mut self, target_height: u64) -> Result<()> {
        if target_height > self.height {
            Err(Error::HeightError)
        } else {
            self.sync_height(target_height, None)
        }
    }

    /// Commit this snapshot.
    pub fn commit(&mut self) -> Result<u64> {
        let mut operations = Vec::new();

        // exchange cache.
        let cache = mem::replace(&mut self.cache, BTreeMap::new());

        for (k, v) in cache {
            let key_bytes = utils::storage_key(self.namespace, &k, self.height);
            let store_value = StoreValue { operation: v.to_operation() };
            operations.push((key_bytes, store_value.to_bytes()?));
        }

        // incr current height
        self.sync_height(self.height + 1, Some(operations))?;

        Ok(self.height)
    }
}

/// Methods for internal helper
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    pub(crate) fn sync_height(&mut self, target_height: u64, pre_commit: Option<Vec<(Vec<u8>, Vec<u8>)>>) -> Result<()> {
        self.height = target_height;

        let mut operations = if let Some(ops) = pre_commit {
            ops
        } else {
            Vec::new()
        };

        let store_height = StoreHeight {
            height: self.height,
        };
        let height_key_bytes = utils::current_height_key(self.namespace);
        let height_value_bytes = store_height.to_bytes()?;
        operations.push((height_key_bytes, height_value_bytes));
        self.store.execute(operations)?;

        Ok(())
    }

    /// Get value in target height directly.
    pub(crate) fn direct_raw_get(&self, key: &Output<D>, height: u64) -> Result<Option<&[u8]>> {
        let key = utils::storage_key(self.namespace, key, height);
        let value = self.store.get_lt(&key)?;
        Ok(value)
    }
}

impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    pub fn get(&self, key: &Output<D>) -> Result<Option<&[u8]>> {
        let cache_result = self.cache.get(key);
        match cache_result {
            Some(OperationOwned::Update(v)) => Ok(Some(v.as_slice())),
            Some(OperationOwned::Delete) => Ok(None),
            None => match self.direct_raw_get(key, self.height)? {
                // I can't write result to cache.
                // If want cache to speed up, in lower `Store`.
                Some(bytes) => {
                    let r = StoreValue::from_bytes(bytes)?;
                    match r.operation {
                        Operation::Update(v) => Ok(Some(&v)),
                        Operation::Delete => Ok(None)
                    }
                }
                None => Ok(None),
            },
        }
    }

    pub fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        if let Some(OperationOwned::Delete) = self.cache.get(key) {
            return Ok(None);
        }
        if self.cache.contains_key(key) {
            let k = utils::storage_key(self.namespace, key, self.height);
            if let Some(bytes) = self.store.get_lt(k.as_slice())? {
                // if has value in cache, same as update.
                let r = StoreValue::from_bytes(bytes)?;
                self.cache.insert(key.clone(), r.operation.to_operation_owned());
            } else {
                return Ok(None);
            }
        }

        // I'm sure this value exists.
        if let OperationOwned::Update(value) = self.cache.get_mut(key).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        if let Some(OperationOwned::Update(v)) = self.cache.insert(key, OperationOwned::Update(value)) {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        if let Some(OperationOwned::Update(v)) = self.cache.insert(key.clone(), OperationOwned::Delete) {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}

