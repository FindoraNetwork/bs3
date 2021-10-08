use core::fmt::Debug;

use crate::merkle::Merkle;
use crate::model::Value;
use crate::{Cow, Operation, Result, Store, Transaction, ValueStore};
use serde::{Deserialize, Serialize};

impl<'a, S, M, T> ValueStore<T> for Transaction<'a, S, M, Value<T>>
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
            None => None,
        })
    }

    fn set(&mut self, value: T) -> Result<Option<T>> {
        return if let Some(operation) = self.value.value.as_ref() {
            match operation {
                Operation::Update(v) => {
                    let v2 = v.clone();
                    self.value.value = Some(Operation::Update(value));
                    Ok(Some(v2))
                }
                Operation::Delete => Ok(None),
            }
        } else {
            self.value.value = Some(Operation::Update(value));
            Ok(None)
        };
    }

    fn del(&mut self) -> Result<Option<T>> {
        return if let Some(operation) = self.value.value.as_ref() {
            match operation {
                Operation::Update(v) => {
                    let v2 = v.clone();
                    self.value.value = Some(Operation::Delete);
                    Ok(Some(v2))
                }
                Operation::Delete => Ok(None),
            }
        } else {
            Ok(None)
        };
    }
}
