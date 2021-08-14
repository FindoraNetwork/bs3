use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::{
    prelude::{Tree, TreeMut},
    snapshot::OperationOwned,
    Result, SnapshotableStorage,
};

pub struct Transaction<'a, D: Digest> {
    pub(crate) store: &'a mut SnapshotableStorage<'a, D>,
    pub(crate) cache: BTreeMap<Output<D>, OperationOwned>,
}

impl<'a, D> Transaction<'a, D>
where
    D: Digest,
{
    pub(crate) fn new(store: &'a mut SnapshotableStorage<'a, D>) -> Self {
        Transaction {
            store,
            cache: BTreeMap::new(),
        }
    }

    fn raw_insert(&mut self, key: &Output<D>, value: OperationOwned) -> Result<Option<Vec<u8>>> {
        Ok(match self.cache.insert(key.clone(), value) {
            Some(OperationOwned::Update(v)) => Some(v),
            Some(OperationOwned::Delete) => None,
            None => match self.store.get(key)? {
                Some(v) => Some(Vec::from(v)),
                None => None,
            },
        })
    }
}

impl<'a, D> Tree<D> for Transaction<'a, D>
where
    D: Digest,
{
    fn get(&self, key: &Output<D>) -> Result<Option<&[u8]>> {
        let cache_result = self.cache.get(key);
        Ok(match cache_result {
            Some(OperationOwned::Update(v)) => Some(v.as_slice()),
            Some(OperationOwned::Delete) => None,
            None => self.store.get(key)?,
        })
    }
}

impl<'a, D> TreeMut<D> for Transaction<'a, D>
where
    D: Digest,
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
