use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};
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
        if let Some(OperationOwned::Update(v)) =
            self.cache.insert(key, OperationOwned::Update(value))
        {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        if let Some(OperationOwned::Update(v)) =
            self.cache.insert(key.clone(), OperationOwned::Delete)
        {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}
