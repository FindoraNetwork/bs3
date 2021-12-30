//!
//! value cache layer

use core::{fmt::Debug, mem};

use alloc::vec::Vec;

use crate::{Operation, OperationBytes, Result};

use super::{Model, ValueType};

/// define value
#[derive(Debug, Clone)]
pub struct Value<T>
where
    T: ValueType,
{
    value: Option<Operation<T>>,
}

impl<T> Value<T>
where
    T: ValueType,
{
    /// crate Value
    pub fn new(t: T) -> Self {
        Self {
            value: Some(Operation::Update(t)),
        }
    }

    /// Store a `T` in it. self.
    pub fn store(&mut self, t: T) {
        self.value = Some(Operation::Update(t));
    }

    /// Take the value it stores. `self.value` will be `None`
    pub fn take(&mut self) -> Option<Operation<T>> {
        self.value.take()
    }

    /// Mark the value it stores as `Delete` if `self.value` is Some(). return previous value.
    pub fn del(&mut self) -> Option<Operation<T>> {
        let res = self.take();
        if res.is_some() {
            self.value = Some(Operation::Delete);
        }
        res
    }

    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    pub fn as_ref(&self) -> Option<&Operation<T>> {
        self.value.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut Operation<T>> {
        self.value.as_mut()
    }
}

impl<T> Default for Value<T>
where
    T: ValueType,
{
    fn default() -> Self {
        Self { value: None }
    }
}

/// impl Model
impl<T> Model for Value<T>
where
    T: ValueType,
{
    /// define type 1
    fn type_code(&self) -> u32 {
        1
    }

    /// Consume the data in the cache
    /// Also convert key to vec<u8>
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>> {
        let mut vec = Vec::with_capacity(1);

        let value = mem::replace(&mut self.value, None);

        if let Some(value) = value {
            // Empty key.
            let key = Vec::new();

            vec.push((key, value.to_bytes()?));
        }

        Ok(vec)
    }

    ///Replacement value
    fn merge(&mut self, other: Self) {
        self.value = other.value
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use sha3::Sha3_512;

    use crate::{backend::MemoryBackend, merkle::empty::EmptyMerkle, SnapshotableStorage};

    use super::Value;

    #[test]
    fn test_value() {
        env_logger::init();
        let value: Value<String> = Value::new(String::from("aaaaaa"));
        let store = MemoryBackend::new();
        let mut storage =
            SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(value, store).unwrap();

        storage.commit().unwrap();
        storage.commit().unwrap();
        std::println!("{:#?}", storage.store());
    }
}
