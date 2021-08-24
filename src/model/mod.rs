use alloc::{collections::BTreeMap, vec::Vec};

use crate::{OperationBytes, Result};

mod value;
pub use value::Value;

mod map;
pub use map::Map;

mod vec;

pub trait Model {
    fn operations(&self) -> Result<BTreeMap<Vec<u8>, OperationBytes>>;
}
