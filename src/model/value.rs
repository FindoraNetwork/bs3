use core::{fmt::Debug, mem};

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Value<T>
where
    T: Debug + Serialize + for<'de> Deserialize<'de>,
{
    value: Option<Operation<T>>,
}

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
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
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
}
