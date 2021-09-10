use core::fmt::Debug;

use super::utils::vec_utils;
use crate::model::Vec;
use crate::{Cow, Operation, Store, Transaction, VecStore};
use serde::{Deserialize, Serialize};

impl<'a, S, T> VecStore<T> for Transaction<'a, S, Vec<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    fn get(&self, index: usize) -> crate::Result<Option<Cow<'_, T>>> {
        if let Some(operation) = self.value.value.get(&index) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            match vec_utils::get_inner_value(self.store, index)? {
                None => Ok(None),
                Some(v) => Ok(Some(Cow::Owned(v))),
            }
        }
    }

    fn get_mut(&mut self, index: usize) -> crate::Result<Option<&mut T>> {
        if let Some(Operation::Delete) = self.value.value.get(&index) {
            return Ok(None);
        }

        if !self.value.value.contains_key(&index) {
            if let Some(operation) = vec_utils::get_inner_operation(self.store, index)? {
                self.value.value.insert(index, operation);
            } else {
                return Ok(None);
            }
        }

        if let Operation::Update(value) = self.value.value.get_mut(&index).unwrap() {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, value: T) -> crate::Result<Option<T>> {
        let operation = Operation::Update(value.clone());
        let index = self.value.value.len();
        self.value.value.insert(index, operation);
        vec_utils::get_inner_value(self.store, index)
    }

    fn remove(&mut self, index: usize) -> crate::Result<Option<T>> {
        self.value.value.remove(&index);
        vec_utils::get_inner_value(self.store, index)
    }
}
