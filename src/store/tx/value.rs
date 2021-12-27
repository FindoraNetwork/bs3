use core::fmt::Debug;

use crate::merkle::Merkle;
use crate::model::{Value, ValueType};
use crate::{Cow, Operation, Result, Store, Transaction, ValueStore};
use serde::{Deserialize, Serialize};

impl<'a, S, M, T> ValueStore<T> for Transaction<'a, S, M, Value<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>> {
        Ok(match self.value.as_ref() {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => self.store.get()?,
        })
    }

    fn get_mut(&mut self) -> Result<Option<&mut T>> {
        Ok(if self.value.is_some() {
            match self.value.as_mut().unwrap() {
                Operation::Update(v) => Some(v),
                Operation::Delete => None,
            }
        } else {
            let t = self.store.get()?.map(Cow::into_owned);
            match t {
                Some(t) => {
                    self.value.store(t);
                    match self.value.as_mut() {
                        Some(Operation::Update(v)) => Some(v),
                        _ => unreachable!(),
                    }
                }
                _ => None,
            }
        })
    }

    fn set(&mut self, value: T) -> Result<Option<T>> {
        let before = self.value.take();
        self.value.store(value);
        Ok(match before {
            Some(Operation::Update(t)) => Some(t),
            _ => None,
        })
    }

    fn del(&mut self) -> Result<Option<T>> {
        let before = self.value.del();
        Ok(match before {
            Some(Operation::Update(t)) => Some(t),
            _ => None,
        })
    }
}
