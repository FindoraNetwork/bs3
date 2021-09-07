use core::fmt::Debug;

use alloc::vec::Vec;
// use serde::{Deserialize, Serialize};
use minicbor::{Decode as Deserialize, Encode as Serialize};

use crate::utils::cbor_encode;
use crate::{
    model::Value,
    snapshot::{FromStoreBytes, StoreValue},
    Cow, Operation, Result, SnapshotableStorage, Store,
};

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
            None => match get_inner_value(self)? {
                Some(v) => Some(Cow::Owned(v)),
                None => None,
            },
        })
    }

    fn set(&mut self, value: T) -> Result<Option<T>> {
        self.value.value = Some(Operation::Update(value));
        get_inner_value(self)
    }

    fn del(&mut self) -> Result<Option<T>> {
        self.value.value = Some(Operation::Delete);
        get_inner_value(self)
    }
}

// impl<'a, T, S> ValueStore<T> for Transaction<'a, S, Value<T>>
// where
//     T: Debug + Serialize + for<'de> Deserialize<'de>,
//     S: Store,
// {
//     fn get(&self) -> Result<Option<Cow<'_, T>>> {
//         Ok(match &self.value.value {
//             Some(v) => match v {
//                 Operation::Update(iv) => Some(Cow::Borrowed(iv)),
//                 Operation::Delete => None,
//             },
//             None => match self.store.get()? {
//                 Some(v) => Some(Cow::Owned(v)),
//                 None => None,
//             },
//         })
//     }
//
//     fn set(&mut self, value: T) -> Result<Option<T>> {
//
//     }
//
//     fn del(&mut self) -> Result<Option<T>> {}
// }

fn storage_key<S, T>(vss: &SnapshotableStorage<S, Value<T>>) -> Vec<u8>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    let inner_key = Vec::new();
    vss.storage_key(&inner_key)
}

fn get_inner_value<S, T>(vss: &SnapshotableStorage<S, Value<T>>) -> Result<Option<T>>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
    S: Store,
{
    let key = storage_key(vss);
    match vss.store.get_ge(&key)? {
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
