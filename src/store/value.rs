use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::utils::value_utils;
use crate::{Cow, Operation, Result, SnapshotableStorage, Store, merkle::Merkle, model::Value};

pub trait ValueStore<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn get(&self) -> Result<Option<Cow<'_, T>>>;

    fn set(&mut self, value: T) -> Result<Option<T>>;

    fn del(&mut self) -> Result<Option<T>>;
}

impl<S, M, T> ValueStore<T> for SnapshotableStorage<S, M, Value<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
    M: Merkle,
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
        let res = if let Some(operation) = self.value.value.as_ref() {
            match operation {
                Operation::Update(v) => {
                    let v2 = v.clone();
                    Some(v2)
                }
                Operation::Delete => None,
            }
        } else {
            if let Some(v) = value_utils::get_inner_value(self)?{
                Some(v)
            } else {
                None
            }
        };

        self.value.value = Some(Operation::Delete);

        Ok(res)
    }
}
