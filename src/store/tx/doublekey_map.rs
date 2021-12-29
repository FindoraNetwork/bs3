use crate::merkle::Merkle;
use crate::model::{DoubleKeyMap, KeyType, ValueType};
use crate::{Cow, DoubleKeyMapStore, Operation, Result, Store, Transaction};
use core::borrow::Borrow;

//use crate::store::utils::doublekeymap_utils;
use alloc::borrow::ToOwned;
use core::fmt::Debug;
#[cfg(feature = "cbor")]
use serde::{Deserialize, Serialize};

impl<'a, S, M, K1, K2, V> DoubleKeyMapStore<K1, K2, V>
    for Transaction<'a, S, M, DoubleKeyMap<K1, K2, V>>
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
            None => match self.store.get_key1(key1)? {
                Some(v) => Some(v),
                None => None,
            },
        })
    }

    fn get_mut_key1<Q>(&mut self, key1: &Q) -> Result<Option<&mut V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K1>,
        K1: Borrow<Q>,
    {
        //may avoid double borrow-mut

        if self.value.contains_opertaion(key1) {
            return Ok(match self.value.get_mut_value(key1).unwrap() {
                Operation::Delete => None,
                Operation::Update(t) => Some(t),
            });
        };

        let value = match self.get_key1(key1)? {
            Some(v) => v.into_owned(),
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
            None => match self.store.key2_to_key1(key2)? {
                Some(key1) => (
                    self.store
                        .get_key1(&key1)?
                        .expect("impl bug, value must exist.")
                        .into_owned(),
                    key1.into_owned(),
                ),
                None => return Ok(None),
            },
        };

        self.value.insert_operation(key1, Operation::Delete);
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
            None => self.store.key2_to_key1(key2)?,
        })
    }
}
