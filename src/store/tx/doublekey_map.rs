use crate::merkle::Merkle;
use crate::model::{DoubleKeyMap, KeyType, ValueType};
use crate::{Cow, DoubleKeyMapStore, Operation, Result, Store, Transaction};
use core::borrow::Borrow;

//use crate::store::utils::doublekeymap_utils;
use alloc::borrow::ToOwned;
use serde::Serialize;

impl<'a, S, M, K1, K2, V> DoubleKeyMapStore<K1, K2, V>
    for Transaction<'a, S, M, DoubleKeyMap<K1, K2, V>>
where
    K1: KeyType,
    K2: KeyType,
    V: ValueType,
    S: Store,
    M: Merkle,
{
    fn get_key2<Q: ?Sized + Ord + Serialize>(&self, key2: &Q) -> Result<Option<Cow<'_, V>>>
    where
        K2: Borrow<Q>,
    {
        Ok(match self.value.get_value(key2) {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => self.store.get_key2(key2)?,
        })
    }

    fn get_mut_key2<Q>(&mut self, key2: &Q) -> Result<Option<&mut V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K2>,
        K2: Borrow<Q>,
    {
        //may avoid double borrow-mut

        if self.value.contains_opertaion(key2) {
            return Ok(match self.value.get_mut_value(key2).unwrap() {
                Operation::Delete => None,
                Operation::Update(t) => Some(t),
            });
        };

        let value = match self.get_key2(key2)? {
            Some(v) => v.into_owned(),
            None => return Ok(None),
        };

        self.value
            .insert_operation(key2.to_owned(), Operation::Update(value));

        Ok(match self.value.get_mut_value(key2) {
            Some(Operation::Update(v)) => Some(v),
            _ => unreachable!(),
        })
    }

    fn insert(&mut self, key1: K1, key2: K2, value: V) -> Result<Option<V>> {
        let v_old = self.remove_by_key1(&key1)?;
        self.value
            .insert_operation(key2.clone(), Operation::Update(value));
        self.value
            .insert_operation_key2(key1, Operation::Update(key2));
        Ok(v_old)
    }

    fn remove_by_key1<Q>(&mut self, key1: &Q) -> Result<Option<V>>
    where
        Q: ?Sized + Ord + Serialize + ToOwned<Owned = K1>,
        K1: Borrow<Q>,
    {
        let (old_value, key2) = match self.value.remove_key2(key1) {
            Some(Operation::Update(k2)) => match self.value.remove_operation(&k2) {
                Some(Operation::Update(v)) => (v, k2),
                _ => unreachable!(),
            },
            Some(Operation::Delete) => return Ok(None),
            None => match self.store.key1_to_key2(key1)? {
                Some(key2) => (
                    self.store
                        .get_key2(&key2)?
                        .expect("impl bug, value must exist.")
                        .into_owned(),
                    key2.into_owned(),
                ),
                None => return Ok(None),
            },
        };

        self.value.insert_operation(key2, Operation::Delete);
        self.value
            .insert_operation_key2(key1.to_owned(), Operation::Delete);

        Ok(Some(old_value))
    }

    fn key1_to_key2<Q: ?Sized + Ord + Serialize>(&self, key1: &Q) -> Result<Option<Cow<'_, K2>>>
    where
        K1: Borrow<Q>,
    {
        Ok(match self.value.get_key2(key1) {
            Some(Operation::Update(key2)) => Some(Cow::Borrowed(key2)),
            Some(Operation::Delete) => None,
            None => self.store.key1_to_key2(key1)?,
        })
    }
}
