use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::{Result, SnapshotableStorage, backend::Store, bytes_ref::BytesRef, prelude::{Tree, TreeMut}, snapshot::{OperationOwned}};

pub struct Transaction<'a, S, D>
where
    D: Digest,
   S: Store,
{
    pub(crate) store: &'a mut SnapshotableStorage<S, D>,
    pub(crate) cache: BTreeMap<Output<D>, OperationOwned>,
}

impl<'a, S, D> Transaction<'a, S, D>
where
    D: Digest,
    S: Store,
{
    pub(crate) fn new(store: &'a mut SnapshotableStorage<S, D>) -> Self {
        Transaction {
            store,
            cache: BTreeMap::new(),
        }
    }

    fn raw_insert(&mut self, key: &Output<D>, value: OperationOwned) -> Result<Option<Vec<u8>>> {
        Ok(match self.cache.insert(key.clone(), value) {
            Some(OperationOwned::Update(v)) => Some(v),
            Some(OperationOwned::Delete) => None,
            None => self.store.get(key)?.map(|v| v.into()),
        })
    }
}

impl<'a, S, D> Tree<D> for Transaction<'a, S, D>
where
    D: Digest,
    S: Store,
{
    fn get(&self, key: &Output<D>) -> Result<Option<BytesRef>> {
        let cache_result = self.cache.get(key);
        Ok(match cache_result {
            Some(OperationOwned::Update(v)) => Some(BytesRef::Borrow(v.as_slice())),
            Some(OperationOwned::Delete) => None,
            None => self.store.get(key)?,
        })
    }
}

impl<'a, S, D> TreeMut<D> for Transaction<'a, S, D>
where
    D: Digest,
    S: Store,
{
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        let cache_result = self.cache.get_mut(key);
        Ok(match cache_result {
            Some(OperationOwned::Update(v)) => Some(v),
            Some(OperationOwned::Delete) => None,
            None => self.store.get_mut(key)?,
        })
    }

    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        self.raw_insert(&key, OperationOwned::Update(value))
    }

    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        self.raw_insert(key, OperationOwned::Delete)
    }
}
