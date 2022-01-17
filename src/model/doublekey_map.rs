use core::borrow::Borrow;
use core::{fmt::Debug, mem};

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::model::{KeyType, Model, ValueType};
use crate::{Operation, OperationBytes, Result};

#[derive(Debug, Clone)]
pub struct DoubleKeyMap<K1, K2, V>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    cache: BTreeMap<K1, Operation<V>>,
    key2key: BTreeMap<K2, Operation<K1>>,
}

impl<K1, K2, V> DoubleKeyMap<K1, K2, V>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    pub fn get_value<Q: ?Sized + Ord>(&self, key1: &Q) -> Option<&Operation<V>>
    where
        K1: Borrow<Q>,
    {
        self.cache.get(key1)
    }

    pub fn get_key1<Q: ?Sized + Ord>(&self, key2: &Q) -> Option<&Operation<K1>>
    where
        K2: Borrow<Q>,
    {
        self.key2key.get(key2)
    }

    pub fn get_mut_value<Q: ?Sized + Ord>(&mut self, key1: &Q) -> Option<&mut Operation<V>>
    where
        K1: Borrow<Q>,
    {
        self.cache.get_mut(key1)
    }

    pub fn get_mut_key1<Q: ?Sized + Ord>(&mut self, key2: &Q) -> Option<&mut Operation<K1>>
    where
        K2: Borrow<Q>,
    {
        self.key2key.get_mut(key2)
    }

    pub fn remove_operation<Q: ?Sized + Ord>(&mut self, key1: &Q) -> Option<Operation<V>>
    where
        K1: Borrow<Q>,
    {
        self.cache.remove(key1)
    }

    pub fn remove_key1<Q: ?Sized + Ord>(&mut self, key2: &Q) -> Option<Operation<K1>>
    where
        K2: Borrow<Q>,
    {
        self.key2key.remove(key2)
    }

    pub fn insert_operation(&mut self, key1: K1, value: Operation<V>) -> Option<Operation<V>> {
        self.cache.insert(key1, value)
    }

    pub fn insert_operation_key1(&mut self, key2: K2, key1: Operation<K1>) {
        self.key2key.insert(key2, key1);
    }

    pub fn contains_opertaion<Q: ?Sized + Ord>(&self, key1: &Q) -> bool
    where
        K1: Borrow<Q>,
    {
        self.cache.contains_key(key1)
    }
}

impl<K1, K2, V> Default for DoubleKeyMap<K1, K2, V>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    fn default() -> Self {
        Self {
            cache: BTreeMap::new(),
            key2key: BTreeMap::new(),
        }
    }
}

impl<K1, K2, V> Model for DoubleKeyMap<K1, K2, V>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        use crate::utils::cbor_encode_writer;

        let mut operations = Vec::new();

        for (k1, v) in mem::take(&mut self.cache).into_iter() {
            let mut key: Vec<u8> = Vec::with_capacity(16);
            key.push(1);
            cbor_encode_writer(&k1, &mut key)?;
            let value = v.to_bytes()?;
            operations.push((key, value));
        }

        for (k2, v) in mem::take(&mut self.key2key).into_iter() {
            let mut key: Vec<u8> = Vec::with_capacity(16);
            key.push(2);
            cbor_encode_writer(&k2, &mut key)?;
            let value = v.to_bytes()?;
            operations.push((key, value));
        }

        Ok(operations)
    }

    fn type_code(&self) -> u32 {
        4
    }

    fn merge(&mut self, mut other: Self) {
        self.cache.append(&mut other.cache);
        self.key2key.append(&mut other.key2key);
    }
}
