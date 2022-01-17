use crate::{
    merkle::Merkle,
    model::{Value, ValueType},
    Cow, Operation, Result, SnapshotableStorage, Store,
};
use alloc::vec::Vec;

use super::utils::get_greatest;

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
            None => self.get_inner_value()?.map(Cow::Owned),
        })
    }

    fn get_mut(&mut self) -> Result<Option<&mut T>> {
        Ok(if self.value.is_some() {
            match self.value.as_mut().unwrap() {
                Operation::Update(t) => Some(t),
                Operation::Delete => None,
            }
        } else {
            let v = self.get_inner_value()?;
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
        self.get_inner_value()
    }

    fn del(&mut self) -> Result<Option<T>> {
        Ok(match self.value.del() {
            Some(operation) => match operation {
                Operation::Update(t) => Some(t),
                Operation::Delete => None,
            },
            None => return self.get_inner_value(),
        })
    }
}

impl<S, M, T> SnapshotableStorage<S, M, Value<T>>
where
    S: Store,
    M: Merkle,
    T: ValueType,
{
    fn get_inner_value(&self) -> Result<Option<T>> {
        let store_key = self.storage_tuple_key(&Vec::new());
        get_greatest(&self.store, &store_key.0, &store_key.1)
    }
}
