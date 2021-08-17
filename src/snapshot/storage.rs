use core::{marker::PhantomData, mem};

use alloc::{collections::{BTreeMap}, string::{String, ToString}, vec::Vec};
use digest::{Digest, Output};

use crate::{
    backend::Store,
    prelude::{Tree, TreeMut},
    Error, Result, Transaction,
};

use super::{Operation, StoreHeight, StoreValue, ToStoreBytes, bytes_ref::BytesRef, operation::OperationOwned, utils};

/// Snapshotable Storage
pub struct SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
{
    store: S,
    height: u64,
    pub(crate) cache: BTreeMap<Output<D>, OperationOwned>,
    namespace: String,
    marker_r: PhantomData<R>,
}

/// Methods for create storage.
impl<S, D, R> SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
{
    pub fn new(store: S) -> Self {
        Self::new_with_name(store, "".to_string())
    }

    pub fn new_with_name(store: S, name: String) -> Self {
        Self {
            store,
            height: 0,
            cache: BTreeMap::new(),
            namespace: name,
            marker_r: PhantomData,
        }
    }

    pub fn new_with_height(store: S, height: u64) -> Result<Self> {
        let mut s = Self::new(store);
        s.rollback(height)?;
        Ok(s)
    }

    pub fn new_with_height_namespace(store: S, height: u64, namespace: String) -> Result<Self> {
        let mut s = Self {
            store: store,
            height,
            cache: BTreeMap::new(),
            namespace,
            marker_r: PhantomData,
        };
        s.rollback(height)?;
        Ok(s)
    }
}

/// Methods for snapshot.
impl<S, D, R> SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
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

        log::debug!("Snapshot Cache: {:#?}", cache);

        for (k, v) in cache {
            let key_bytes = utils::storage_key(&self.namespace, &k, self.height);
            let store_value = StoreValue {
                operation: v.to_operation(),
            };
            operations.push((key_bytes, store_value.to_bytes()?));
        }

        // incr current height
        self.sync_height(self.height + 1, Some(operations))?;

        log::debug!("Sync snapshot success in height: {}", self.height);

        Ok(self.height)
    }
}

/// Methods for transaction
impl<S, D, R> SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
{
    /// Generate transaction for this Bs3 db.
    pub fn transaction(&mut self) -> Transaction<S, D, R> {
        Transaction::new(self)
    }

    /// Consume transaction to apply.
    pub fn execute(&mut self, mut tx: Transaction<S, D, R>) {
        log::debug!("Transaction Cache: {:#?}", tx.cache);
        self.cache.append(&mut tx.cache);
    }
}

/// Methods for internal helper
impl<S, D, R> SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
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
        let height_key_bytes = utils::current_height_key(&self.namespace);
        let height_value_bytes = store_height.to_bytes()?;
        operations.push((height_key_bytes, height_value_bytes));
        self.store.execute(operations)?;

        Ok(())
    }

    /// Get value in target height directly.
    pub(crate) fn raw_get_lt(&self, key: &Output<D>, height: u64) -> Result<Option<Vec<u8>>> {
        let end_key = utils::storage_key(&self.namespace, key, height);
        let begin_key = utils::storage_key(&self.namespace, key, 0);
        let mut value = self.store.range(begin_key, end_key)?;
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
                    let r = StoreValue::from_bytes(&bytes)?;
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

impl<S, D, R> Tree<D> for SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
{
    fn get(&self, key: &Output<D>) -> Result<Option<BytesRef<'_>>> {
        let cache_result = self.cache.get(key);
        match cache_result {
            Some(OperationOwned::Update(v)) => Ok(Some(BytesRef::Borrow(v.as_slice()))),
            Some(OperationOwned::Delete) => Ok(None),
            None => match self.raw_get_lt(key, self.height)? {
                // I can't write result to cache.
                // If want cache to speed up, in lower `Store`.
                Some(bytes) => {
                    let r = StoreValue::from_bytes(&bytes)?;
                    match r.operation {
                        // this need optine, beacuse this cause copy
                        Operation::Update(v) => Ok(Some(BytesRef::Owned(Vec::from(v)))),
                        Operation::Delete => Ok(None),
                    }
                }
                None => Ok(None),
            },
        }
    }
}

impl<S, D, R> TreeMut<D> for SnapshotableStorage<S, D, R>
where
    D: Digest,
    R: Iterator<Item = (Vec<u8>, Vec<u8>)>,
    S: Store<Range = R>,
{
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        if let Some(OperationOwned::Delete) = self.cache.get(key) {
            return Ok(None);
        }
        if self.cache.contains_key(key) {
            match self.raw_get_lt(key, self.height)? {
                Some(bytes) => {
                    let r = StoreValue::from_bytes(&bytes)?;
                    // this assign to prevent #[warn(mutable_borrow_reservation_conflict)]
                    let operation_owned = r.operation.to_operation_owned();
                    self.cache.insert(key.clone(), operation_owned);
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
