use core::fmt::Debug;

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
            Ok(None)
        }
    }

    fn get_mut(&mut self, index: usize) -> crate::Result<Option<&mut T>> {
        if let Operation::Update(value) = self.value.value.get_mut(&index).unwrap() {
            Ok(Some(value))
        } else {
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
