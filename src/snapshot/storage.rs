use core::mem;

use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::{
    backend::Store,
    prelude::{Tree, TreeMut},
    Error, Result, Transaction,
};

use super::{operation::OperationOwned, utils, Operation, StoreHeight, StoreValue, ToStoreBytes};

/// Snapshotable Storage
pub struct SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    store: &'a dyn Store<'a, Range = R>,
    height: u64,
    pub(crate) cache: BTreeMap<Output<D>, OperationOwned>,
    namespace: &'a str,
}

/// Methods for create storage.
impl<'a, D, R> SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    pub fn new(store: &'a impl Store<'a, Range = R>) -> Self {
        Self::new_with_name(store, "")
    }

    pub fn new_with_name(store: &'a impl Store<'a, Range = R>, name: &'a str) -> Self {
        Self {
            store: store as &'a dyn Store<Range = R>,
            height: 0,
            cache: BTreeMap::new(),
            namespace: name,
        }
    }

    pub fn new_with_height(store: &'a impl Store<'a, Range = R>, height: u64) -> Result<Self> {
        let mut s = Self::new(store);
        s.rollback(height)?;
        Ok(s)
    }

    pub fn new_with_height_namespace(
        store: &'a impl Store<'a, Range = R>,
        height: u64,
        namespace: &'a str,
    ) -> Result<Self> {
        let mut s = Self {
            store: store as &'a dyn Store<Range = R>,
            height,
            cache: BTreeMap::new(),
            namespace,
        };
        s.rollback(height)?;
        Ok(s)
    }
}

/// Methods for snapshot.
impl<'a, D, R> SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
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
            let store_value = StoreValue {
                operation: v.to_operation(),
            };
            operations.push((key_bytes, store_value.to_bytes()?));
        }

        // incr current height
        self.sync_height(self.height + 1, Some(operations))?;

        Ok(self.height)
    }
}

/// Methods for transaction
impl<'a, D, R> SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    /// Generate transaction for this Bs3 db.
    pub fn transaction(&'a mut self) -> Transaction<'a, D, R> {
        Transaction::new(self)
    }

    /// Consume transaction to apply.
    pub fn execute(&'a mut self, mut tx: Transaction<'a, D, R>) {
        self.cache.append(&mut tx.cache);
    }
}

/// Methods for internal helper
impl<'a, D, R> SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    pub(crate) fn sync_height(
        &mut self,
        target_height: u64,
        pre_commit: Option<Vec<(Vec<u8>, Vec<u8>)>>,
    ) -> Result<()> {
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
    pub(crate) fn raw_get_lt(&self, key: &Output<D>, height: u64) -> Result<Option<&[u8]>> {
        let end_key = utils::storage_key(self.namespace, key, height);
        let begin_key = utils::storage_key(self.namespace, key, 0);
        let mut value = self.store.range(&begin_key, &end_key)?;
        Ok(match value.next() {
            Some((_, v)) => Some(v),
            None => None,
        })
    }

    fn raw_insert(&mut self, key: &Output<D>, value: OperationOwned) -> Result<Option<Vec<u8>>> {
        Ok(match self.cache.insert(key.clone(), value) {
            Some(OperationOwned::Update(v)) => Some(v),
            Some(OperationOwned::Delete) => None,
            None => match self.raw_get_lt(key, self.height)? {
                Some(bytes) => {
                    let r = StoreValue::from_bytes(bytes)?;
                    match r.operation {
                        Operation::Update(v) => Some(Vec::from(v)),
                        Operation::Delete => None,
                    }
                }
                None => None,
            },
        })
    }
}

impl<'a, D, R> Tree<D> for SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    fn get(&self, key: &Output<D>) -> Result<Option<&[u8]>> {
        let cache_result = self.cache.get(key);
        match cache_result {
            Some(OperationOwned::Update(v)) => Ok(Some(v.as_slice())),
            Some(OperationOwned::Delete) => Ok(None),
            None => match self.raw_get_lt(key, self.height)? {
                // I can't write result to cache.
                // If want cache to speed up, in lower `Store`.
                Some(bytes) => {
                    let r = StoreValue::from_bytes(bytes)?;
                    match r.operation {
                        Operation::Update(v) => Ok(Some(&v)),
                        Operation::Delete => Ok(None),
                    }
                }
                None => Ok(None),
            },
        }
    }
}

impl<'a, D, R> TreeMut<D> for SnapshotableStorage<'a, D, R>
where
    D: Digest,
    R: Iterator<Item = (&'a [u8], &'a [u8])>,
{
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        if let Some(OperationOwned::Delete) = self.cache.get(key) {
            return Ok(None);
        }
        if self.cache.contains_key(key) {
            match self.raw_get_lt(key, self.height)? {
                Some(bytes) => {
                    let r = StoreValue::from_bytes(bytes)?;
                    // this assign to prevent #[warn(mutable_borrow_reservation_conflict)]
                    let operation_owned = r.operation.to_operation_owned();
                    self.cache
                        .insert(key.clone(), operation_owned);
                }
                None => return Ok(None),
            }
        }

        // I'm sure this value exists.
        if let OperationOwned::Update(value) = self.cache.get_mut(key).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.raw_insert(&key, OperationOwned::Update(value))
    }

    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        self.raw_insert(key, OperationOwned::Delete)
    }
}
