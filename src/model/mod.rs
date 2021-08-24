use core::fmt::Debug;

use alloc::vec::Vec;

use crate::{OperationBytes, Result};

mod value;
pub use value::Value;

mod map;
pub use map::Map;

mod vec;

pub trait Model: Default + Debug {
    /// Get operations for this value.
    ///
    /// Don't forget clean this value to default.
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>>;

    fn type_code(&self) -> u32;
}
