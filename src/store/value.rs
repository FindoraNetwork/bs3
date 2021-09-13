use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::utils::value_utils;
use crate::{model::Value, Cow, Operation, Result, SnapshotableStorage, Store};

pub trait ValueStore<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>>;

    fn set(&mut self, value: T) -> Result<Option<T>>;

    fn del(&mut self) -> Result<Option<T>>;
}

impl<T, S> ValueStore<T> for SnapshotableStorage<S, Value<T>>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>> {
        Ok(match &self.value.value {
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

    fn set(&mut self, value: T) -> Result<Option<T>> {
        self.value.value = Some(Operation::Update(value));
        value_utils::get_inner_value(self)
    }

    fn del(&mut self) -> Result<Option<T>> {
        self.value.value = Some(Operation::Delete);
        value_utils::get_inner_value(self)
    }
}
