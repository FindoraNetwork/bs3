use core::fmt::Debug;

use crate::merkle::Merkle;
use crate::model::{ValueType, Vec};
use crate::{Cow, Operation, Result, Store, Transaction, VecStore};
use serde::{Deserialize, Serialize};

impl<'a, S, M, T> VecStore<T> for Transaction<'a, S, M, Vec<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn len(&self) -> Result<u64> {
        if let Some(l) = self.value.len() {
            return Ok(l);
        }
        Ok(self.store.len()?)
    }

    fn push(&mut self, value: T) -> Result<()> {
        let len = self.len()?;
        self.value.insert_operation(len, Operation::Update(value));
        self.value.set_len(len + 1);
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<T>> {
        let len = self.len()?;
        if len == 0 {
            return Ok(None);
        }

        self.value.set_len(len - 1);

        match self.value.remove_operation(len - 1) {
            Some(Operation::Update(t)) => {
                return Ok(Some(t));
            }
            _ => (),
        };

        let res = self.store.get(len - 1)?.map(Cow::into_owned);

        match res {
            Some(t) => Ok(Some(t)),
            _ => unreachable!(),
        }
    }

    fn get(&self, index: u64) -> Result<Option<Cow<'_, T>>> {
        let len = self.len()?;
        if index >= len || len == 0 {
            return Ok(None);
        }

        if let Some(operation) = self.value.get(&index) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            match self.store.get(index)? {
                None => Ok(None),
                Some(v) => Ok(Some(v)),
            }
        }
    }

    fn get_mut(&mut self, index: u64) -> Result<Option<&mut T>> {
        let len = self.len()?;
        if index >= len || len == 0 {
            return Ok(None);
        }

        if self.value.contains_key(&index) {
            match self.value.get_mut(&index).unwrap() {
                Operation::Delete => Ok(None),
                Operation::Update(t) => Ok(Some(t)),
            }
        } else {
            match self.store.get(index)? {
                Some(t) => {
                    self.value
                        .insert_operation(index, Operation::Update(t.into_owned()));
                    match self.value.get_mut(&index).unwrap() {
                        Operation::Update(t) => Ok(Some(t)),
                        _ => unreachable!(),
                    }
                }
                None => Ok(None),
            }
        }
    }
}
