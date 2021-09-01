use core::{fmt::Debug, mem};

use alloc::vec::Vec;
// use serde::{Deserialize, Serialize};

#[cfg(feature = "cbor")]
use minicbor::{Decode as Deserialize, Encode as Serialize};

use crate::{Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Value<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub(crate) value: Option<Operation<T>>,
}

impl<T> Value<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(t: T) -> Self {
        Self {
            value: Some(Operation::Update(t)),
        }
    }
}

// pub trait ValueAccess<T> {
//     fn get(&self) -> &Option<Operation<T>>;
//
//     fn set(&mut self, value: T);
//
//     fn del(&mut self);
//
//     fn storage_key(&self) -> Vec<u8>;
// }
//
// impl<T> ValueAccess<T> for Value<T>
// where
//     T: Debug + Serialize + for<'de> Deserialize<'de>,
// {
//     fn get(&self) -> &Option<Operation<T>> {
//         &self.value
//     }
//
//     fn set(&mut self, value: T) {
//         self.value = Some(Operation::Update(value))
//     }
//
//     fn del(&mut self) {
//         self.value = Some(Operation::Delete)
//     }
//
//     fn storage_key(&self) -> Vec<u8> {
//         Vec::new()
//     }
// }

impl<T> Default for Value<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> Model for Value<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn type_code(&self) -> u32 {
        1
    }

    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        let mut vec = Vec::new();

        let value = mem::replace(&mut self.value, None);

        if let Some(value) = value {
            // Empty key.
            let key = Vec::new();

            vec.push((key, value.to_bytes()?));
        }

        Ok(vec)
    }

    fn merge(&mut self, other: Self) {
        self.value = other.value
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::{backend::MemoryBackend, SnapshotableStorage};

    use super::Value;

    #[test]
    fn test_value() {
        env_logger::init();
        let value = Value::new(String::from("aaaaaa"));
        let store = MemoryBackend::new();
        let mut storage = SnapshotableStorage::new(value, store).unwrap();
        storage.commit().unwrap();
        storage.commit().unwrap();
        std::println!("{:#?}", storage.store());
    }
}
