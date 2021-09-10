use core::fmt::Debug;

use super::utils::value_utils;
use crate::model::Value;
use crate::{Cow, Operation, Store, Transaction, ValueStore};
use serde::{Deserialize, Serialize};

impl<'a, S, T> ValueStore<T> for Transaction<'a, S, Value<T>>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    fn get(&self) -> crate::Result<Option<Cow<'_, T>>> {
        Ok(match &self.value.value {
            Some(v) => match v {
                Operation::Update(iv) => Some(Cow::Borrowed(iv)),
                Operation::Delete => None,
            },
            None => match value_utils::get_inner_value(self.store)? {
                Some(v) => Some(Cow::Owned(v)),
                None => None,
            },
        })
    }

    fn set(&mut self, value: T) -> crate::Result<Option<T>> {
        self.value.value = Some(Operation::Update(value));
        value_utils::get_inner_value(self.store)
    }

    fn del(&mut self) -> crate::Result<Option<T>> {
        self.value.value = Some(Operation::Delete);
        value_utils::get_inner_value(self.store)
    }
}
