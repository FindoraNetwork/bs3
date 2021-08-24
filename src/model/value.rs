use core::fmt::Debug;

use alloc::{collections::BTreeMap, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{Operation, OperationBytes, Result};

use super::Model;

#[derive(Debug)]
pub struct Value<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    value: Operation<T>,
}

impl<T> Value<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    // pub fn get(&self) -> Result<Cow<'a, T>> {}
}

impl<T> Model for Value<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    fn operations(&self) -> Result<BTreeMap<Vec<u8>, OperationBytes>> {
        let mut map = BTreeMap::new();

        // Empty key.
        let key = Vec::new();

        map.insert(key, self.value.to_bytes()?);

        Ok(map)
    }
}
