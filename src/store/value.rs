use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::utils::value_utils;
use crate::{
    merkle::Merkle,
    model::{Value, ValueType},
    Cow, Operation, Result, SnapshotableStorage, Store,
};

pub trait ValueStore<T>
where
    T: ValueType,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>>;

    fn get_mut(&mut self) -> Result<Option<&mut T>>;

    fn set(&mut self, value: T) -> Result<Option<T>>;

    fn del(&mut self) -> Result<Option<T>>;
}

impl<S, M, T> ValueStore<T> for SnapshotableStorage<S, M, Value<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>> {
        Ok(match self.value.as_ref() {
            Some(v) => match v {
                Operation::Update(iv) => Some(Cow::Borrowed(iv)),
                Operation::Delete => None,
            },
            None => match value_utils::get_inner_value(self)? {
                Some(v) => Some(Cow::Owned(v)),
                None => None,
            },
        })
    }

    fn get_mut(&mut self) -> Result<Option<&mut T>> {
        Ok(if self.value.is_some() {
            match self.value.as_mut().unwrap() {
                Operation::Update(t) => Some(t),
                Operation::Delete => None,
            }
        } else {
            let v = value_utils::get_inner_value(self)?;
            match v {
                None => None,
                Some(v) => {
                    self.value.store(v);
                    match self.value.as_mut() {
                        Some(Operation::Update(v)) => Some(v),
                        _ => unreachable!(),
                    }
                }
            }
        })
    }

    fn set(&mut self, value: T) -> Result<Option<T>> {
        self.value.store(value);
        value_utils::get_inner_value(self)
    }

    fn del(&mut self) -> Result<Option<T>> {
        Ok(match self.value.del() {
            Some(operation) => match operation {
                Operation::Update(t) => Some(t),
                Operation::Delete => None,
            },
            None => return value_utils::get_inner_value(self),
        })
    }
}
