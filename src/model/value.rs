//!
//! value cache layer

use core::{fmt::Debug, mem};

use alloc::vec::Vec;

use crate::{Operation, OperationBytes, Result};

use super::{Model, ValueT};

/// define value
#[derive(Debug, Clone)]
pub struct Value<T>
where
    T: ValueT,
{
    pub(crate) value: Option<Operation<T>>,
}

impl<T> Value<T>
where
    T: ValueT,
{
    /// crate Value
    pub fn new(t: T) -> Self {
        Self {
            value: Some(Operation::Update(t)),
        }
    }
}

impl<T> Default for Value<T>
where
    T: ValueT,
{
    fn default() -> Self {
        Self { value: None }
    }
}

/// impl Model
impl<T> Model for Value<T>
where
    T: ValueT,
{
    /// define type 1
    fn type_code(&self) -> u32 {
        1
    }

    /// Consume the data in the cache
    /// Also convert key to vec<u8>
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

    ///Replacement value
    fn merge(&mut self, other: Self) {
        self.value = other.value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_value() {
        env_logger::init();
    }
}
