use crate::snapshot::{FromStoreBytes, StoreValue};
use crate::utils::cbor_encode;
use crate::{
    model::{Map, Vec},
    Cow, Operation, Result, SnapshotableStorage, Store,
};
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait VecStore<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn get(&self, index: usize) -> Result<Option<Cow<'_, T>>>;

    fn get_mut(&mut self, index: usize) -> Result<Option<&mut T>>;

    fn insert(&mut self, value: T) -> Result<Option<T>>;

    fn remove(&mut self, index: usize) -> Result<Option<T>>;
}

impl<S, T> VecStore<T> for SnapshotableStorage<S, Vec<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    fn get(&self, index: usize) -> Result<Option<Cow<'_, T>>> {
        if let Some(operation) = self.value.value.get(&index) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            match get_inner_value(self, index)? {
                None => Ok(None),
                Some(v) => Ok(Some(Cow::Owned(v))),
            }
        }
    }

    fn get_mut(&mut self, index: usize) -> Result<Option<&mut T>> {
        if let Some(Operation::Delete) = self.value.value.get(&index) {
            return Ok(None);
        }

        if !self.value.value.contains_key(&index) {
            if let Some(operation) = get_inner_operation(self, index)? {
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

    fn insert(&mut self, value: T) -> Result<Option<T>> {
        let operation = Operation::Update(value.clone());
        let index = self.value.value.len();
        self.value.value.insert(index, operation);
        get_inner_value(self, index)
    }

    fn remove(&mut self, index: usize) -> Result<Option<T>> {
        self.value.value.remove(&index);
        get_inner_value(self, index)
    }
}

fn get_inner_value<S, T>(vss: &SnapshotableStorage<S, Vec<T>>, index: usize) -> Result<Option<T>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    let operation = get_inner_operation(vss, index)?;
    if let Some(operation) = operation {
        match operation {
            Operation::Update(v) => Ok(Some(v)),
            Operation::Delete => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn get_inner_operation<S, T>(
    vss: &SnapshotableStorage<S, Vec<T>>,
    key: usize,
) -> Result<Option<Operation<T>>>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    let key_bytes = cbor_encode(key)?;
    let store_key = vss.storage_key(&key_bytes);
    let bytes = vss.store.get_ge(&*store_key)?;
    if let Some(bytes) = bytes {
        let value = StoreValue::from_bytes(&bytes)?;
        let operation = Operation::from_bytes(&value.operation)?;
        Ok(Some(operation))
    } else {
        Ok(None)
    }
}
