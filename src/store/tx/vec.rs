use core::fmt::Debug;

use crate::model::Vec;
use crate::{Cow, Operation, Store, Transaction, VecStore};
use serde::{Deserialize, Serialize};
use crate::store::utils::vec_utils;

impl<'a, S, T> VecStore<T> for Transaction<'a, S, Vec<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    fn get(&self, index: usize) -> crate::Result<Option<Cow<'_, T>>> {
        let self_value = self.value.value.get(&index);

        Ok(match self_value {
            Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
            Some(Operation::Delete) => None,
            None => {
                let lower_value = self.store.value.value.get(&index);
                match lower_value {
                    Some(Operation::Update(v)) => Some(Cow::Borrowed(v)),
                    Some(Operation::Delete) => None,
                    None => None
                }
            }
        })
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

        // I'm ensure here has value.
        if let Some(Operation::Update(v)) = self.value.value.get_mut(&index) {
            Ok(Some(v))
        } else {
            // So this branch will never enter.
            Ok(None)
        }
    }

    fn insert(&mut self, value: T) -> crate::Result<Option<T>> {
        let operation = Operation::Update(value.clone());
        let index = self.value.value.len();
        let mut pre_val = None;
        if let Some(operation) = self.value.value.get_mut(&index) {
            match operation {
                Operation::Update(v) => {
                    pre_val = Some(v.clone());
                }
                Operation::Delete => {}
            }
        }
        self.value.value.insert(index, operation);
        Ok(pre_val)
    }

    fn remove(&mut self, index: usize) -> crate::Result<Option<T>> {
        return if let Some(op) = self.value.value.remove(&index) {
            match op {
                Operation::Update(v) => Ok(Some(v)),
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        };
    }
}
