use core::{fmt::Debug};

use alloc::vec::Vec;
// use serde::{Deserialize, Serialize};
use minicbor::{Encode as Serialize, Decode as Deserialize};

use crate::{
    model::{self, Value},
    snapshot::{FromStoreBytes, StoreValue},
    Cow, Operation, Result, SnapshotableStorage, Store,
};

pub struct ValueSnapshot<T, S>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    pub(crate) storage: SnapshotableStorage<S, model::Value<T>>,
}

impl<T, S> ValueSnapshot<T, S>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    pub fn new(storage: SnapshotableStorage<S, Value<T>>) -> Self {
        Self { storage }
    }

    fn storage_key(&self) -> Vec<u8> {
        let inner_key = Vec::new();
        self.storage.storage_key(&inner_key)
    }

    pub fn get(&self) -> Result<Option<Cow<'_, T>>> {
        Ok(match &self.storage.value.value {
            Some(v) => match v {
                Operation::Update(iv) => Some(Cow::Borrowed(iv)),
                Operation::Delete => None,
            },
            None => match self.get_inner_value()? {
                Some(v) => Some(Cow::Owned(v)),
                None => None,
            },
        })
    }

    fn get_inner_value(&self) -> Result<Option<T>> {
        let key = self.storage_key();
        match self.storage.store.get_ge(&key)? {
            Some(bytes) => {
                let value = StoreValue::from_bytes(&bytes)?;
                let operation = Operation::from_bytes(&value.operation)?;
                match operation {
                    Operation::Update(v) => Ok(Some(v)),
                    Operation::Delete => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    pub fn set(&mut self, value: T) -> Result<Option<T>> {
        self.storage.value.value = Some(Operation::Update(value));
        self.get_inner_value()
    }

    pub fn del(&mut self) -> Result<Option<T>> {
        self.storage.value.value = Some(Operation::Delete);
        self.get_inner_value()
    }
}
