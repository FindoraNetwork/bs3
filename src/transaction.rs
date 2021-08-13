use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::{
    backend::{Operation, Store},
    prelude::{Tree, TreeMut},
    Result,
};

pub struct Transaction<'a, D: Digest, S: Store> {
    pub(crate) store: &'a S,
    pub(crate) cache: BTreeMap<Output<D>, Operation>,
}

impl<'a, D, S> Transaction<'a, D, S>
where
    D: Digest,
    S: Store,
{
    pub(crate) fn new(store: &'a S) -> Self {
        Transaction {
            store,
            cache: BTreeMap::new(),
        }
    }

    pub(crate) fn to_operations(&self) -> Vec<(&[u8], &Operation)> {
        let mut operations = Vec::new();
        for (k, v) in &self.cache {
            operations.push((k.as_ref(), v))
        }
        operations
    }
}

impl<'a, D, S> Tree<D> for Transaction<'a, D, S>
where
    D: Digest,
    S: Store,
{
    fn get(&self, key: &Output<D>) -> Result<Option<Cow<'_, [u8]>>> {
        let cache_result = self.cache.get(key);
        if let Some(Operation::Update(v)) = cache_result {
            // if insert or update to cache, get this value.
            Ok(Some(Cow::Borrowed(v.as_ref())))
        } else if let Some(Operation::Delete) = cache_result {
            // if delete this value in cache, can't get this value.
            Ok(None)
        } else if let Some(v) = self.store.get(key)? {
            // if cache has no record.
            Ok(Some(Cow::Owned(v)))
        } else {
            Ok(None)
        }
    }
}

impl<'a, D, S> TreeMut<D> for Transaction<'a, D, S>
where
    D: Digest,
    S: Store,
{
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        if let Some(Operation::Delete) = self.cache.get(key) {
            return Ok(None);
        }
        if self.cache.contains_key(key) {
            if let Some(value) = self.store.get(key)? {
                // if has value in cache, same as update.
                self.cache.insert(key.clone(), Operation::Update(value));
            } else {
                return Ok(None);
            }
        }

        // I'm sure this value exists.
        if let Operation::Update(value) = self.cache.get_mut(key).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        if let Some(Operation::Update(v)) = self.cache.insert(key.clone(), Operation::Update(value))
        {
            Ok(Some(v))
        } else if let Some(v) = self.store.get(&key)? {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        let remove_result = self.cache.insert(key.clone(), Operation::Delete);
        let res = if let Some(Operation::Update(value)) = remove_result {
            Ok(Some(value))
        } else if let Some(Operation::Delete) = remove_result {
            Ok(None)
        } else if let Some(value) = self.store.get(key)? {
            Ok(Some(value))
        } else {
            Ok(None)
        };
        res
    }
}
