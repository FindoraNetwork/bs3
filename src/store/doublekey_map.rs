use alloc::{borrow::ToOwned, vec::Vec};
use core::{borrow::Borrow, fmt::Debug};

use crate::{
    merkle::Merkle,
    model::{DoubleKeyMap, KeyType, ValueType},
    operation,
    snapshot::{FromStoreBytes, SnapshotableStorage, StoreValue},
    store::utils::get_greatest,
    utils::{cbor_encode, cbor_encode_writer},
    Cow, Operation, Result, Store,
};

#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

pub trait DoubleKeyMapStore<K1, K2, V>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    fn get_key1<Q: ?Sized + Ord + Serialize>(&self, key1: &Q) -> Result<Option<Cow<'_, V>>>
    where
        K1: Borrow<Q>;

    fn get_mut_key1<Q>(&mut self, key1: &Q) -> Result<Option<&mut V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K1>,
        K1: Borrow<Q>;

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> Result<Option<V>>;

    fn remove_by_key2<Q>(&mut self, key: &Q) -> Result<Option<V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K2>,
        K2: Borrow<Q>;

    fn key2_to_key1<Q: ?Sized + Ord + Serialize>(&self, key2: &Q) -> Result<Option<Cow<'_, K1>>>
    where
        K2: Borrow<Q>;

    /*get for free: */

    fn get_key2<Q: ?Sized + Ord + Serialize>(&self, key2: &Q) -> Result<Option<Cow<'_, V>>>
    where
        K2: Borrow<Q>,
    {
        match self.key2_to_key1(key2.borrow())? {
            Some(key1) => self.get_key1(&key1),
            _ => Ok(None),
        }
    }

    fn get_mut_key2<Q: ?Sized + Ord + Serialize>(&mut self, key2: &Q) -> Result<Option<&mut V>>
    where
        K2: Borrow<Q>,
    {
        let key1 = match self.key2_to_key1(key2)? {
            Some(key1) => key1.into_owned(),
            _ => return Ok(None),
        };
        self.get_mut_key1(&key1)
    }

    fn get<Q1, Q2>(&self, key1: &Q1, key2: &Q2) -> Result<Option<Cow<'_, V>>>
    where
        Q1: ?Sized + Ord + Serialize,
        Q2: ?Sized + Ord + Serialize,
        K1: Borrow<Q1>,
        K2: Borrow<Q2>,
    {
        let key1_load = match self.key2_to_key1(key2)? {
            Some(k) => k,
            None => return Ok(None),
        };

        if key1 != key1_load.as_ref().borrow() {
            return Ok(None);
        }
        //after here, key1 correlates with key2.
        self.get_key1(key1)
    }

    fn get_mut<Q1, Q2>(&mut self, key1: &Q1, key2: &Q2) -> Result<Option<&mut V>>
    where
        Q1: ?Sized + Ord + Serialize + PartialOrd + ToOwned<Owned = K1>,
        Q2: ?Sized + Ord + Serialize + PartialOrd,
        K1: Borrow<Q1>,
        K2: Borrow<Q2>,
    {
        let key1_load = match self.key2_to_key1(key2)? {
            Some(k) => k,
            None => return Ok(None),
        };

        if key1 != (&*key1_load).borrow() {
            return Ok(None);
        }
        self.get_mut_key1(key1)
    }

    fn remove<Q1, Q2>(&mut self, key1: &Q1, key2: &Q2) -> Result<Option<V>>
    where
        Q1: ?Sized + Ord + Serialize + PartialOrd,
        Q2: ?Sized + Ord + Serialize + ToOwned<Owned = K2>,
        K1: Borrow<Q1>,
        K2: Borrow<Q2>,
    {
        let key1_load = match self.key2_to_key1(key2)? {
            Some(k) => k,
            None => return Ok(None),
        };

        if key1 != key1_load.as_ref().borrow() {
            return Ok(None);
        }

        self.remove_by_key2(key2)
    }
}

impl<S, M, K1, K2, V> DoubleKeyMapStore<K1, K2, V>
    for SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
    S: Store,
    M: Merkle,
{
    fn get_key1<Q: ?Sized + Ord + Serialize>(&self, key1: &Q) -> Result<Option<Cow<'_, V>>>
    where
        K1: Borrow<Q>,
    {
        Ok(match self.value.get_value(key1) {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => match self.get_value_inner(&key1)? {
                Some(v) => Some(Cow::Owned(v)),
                None => None,
            },
        })
    }

    fn get_mut_key1<Q: ?Sized>(&mut self, key1: &Q) -> Result<Option<&mut V>>
    where
        Q: Ord + Serialize + ToOwned<Owned = K1>,
        K1: Borrow<Q>,
    {
        //may avoid double borrow-mut

        if self.value.contains_opertaion(key1) {
            return Ok(match self.value.get_mut_value(key1).unwrap() {
                Operation::Delete => None,
                Operation::Update(t) => Some(t),
            });
        };

        let value = match self.get_value_inner(key1)? {
            Some(v) => v,
            None => return Ok(None),
        };

        self.value
            .insert_operation(key1.to_owned(), Operation::Update(value));

        Ok(match self.value.get_mut_value(key1) {
            Some(Operation::Update(v)) => Some(v),
            _ => unreachable!(),
        })
    }

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> Result<Option<V>> {
        let v_old = self.remove_by_key2(&key2)?;
        self.value
            .insert_operation(key1.clone(), Operation::Update(value));
        self.value
            .insert_operation_key1(key2, Operation::Update(key1));
        Ok(v_old)
    }

    fn remove_by_key2<Q>(&mut self, key2: &Q) -> Result<Option<V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K2>,
        K2: Borrow<Q>,
    {
        let (old_value, key1) = match self.value.remove_key1(key2) {
            Some(Operation::Update(k1)) => match self.value.remove_operation(&k1) {
                Some(Operation::Update(v)) => (v, k1),
                _ => unreachable!(),
            },
            Some(Operation::Delete) => return Ok(None),
            None => match self.get_key1_inner(key2)? {
                Some(k1) => (
                    self.get_value_inner(&k1)?
                        .expect("impl bug, value must exist."),
                    k1,
                ),
                None => return Ok(None),
            },
        };

        self.value.insert_operation(key1.clone(), Operation::Delete);
        self.value
            .insert_operation_key1(key2.to_owned(), Operation::Delete);

        Ok(Some(old_value))
    }

    fn key2_to_key1<Q: ?Sized + Ord + Serialize>(&self, key2: &Q) -> Result<Option<Cow<'_, K1>>>
    where
        K2: Borrow<Q>,
    {
        Ok(match self.value.get_key1(key2) {
            Some(Operation::Update(key1)) => Some(Cow::Borrowed(key1)),
            Some(Operation::Delete) => None,
            None => match self.get_key1_inner(key2)? {
                Some(key1) => Some(Cow::Owned(key1)),
                None => None,
            },
        })
    }
}

impl<S, M, K1, K2, V> SnapshotableStorage<S, M, DoubleKeyMap<K1, K2, V>>
where
    S: Store,
    M: Merkle,
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
{
    fn get_value_inner(&self, key1: impl Serialize) -> Result<Option<V>> {
        let mut key_bytes: Vec<u8> = Vec::with_capacity(16);
        key_bytes.push(1);
        cbor_encode_writer(&key1, &mut key_bytes)?;
        let store_key = self.storage_tuple_key(&key_bytes);
        Ok(get_greatest(&self.store, &store_key.0, &store_key.1)?)
    }

    fn get_key1_inner(&self, key2: impl Serialize) -> Result<Option<K1>> {
        let mut key_bytes: Vec<u8> = Vec::with_capacity(16);
        key_bytes.push(2);
        cbor_encode_writer(&key2, &mut key_bytes)?;
        let store_key = self.storage_tuple_key(&key_bytes);
        Ok(get_greatest(&self.store, &store_key.0, &store_key.1)?)
    }
}
