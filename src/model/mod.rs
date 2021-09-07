use core::fmt::Debug;

use alloc::vec::Vec as alloc_vec;

use crate::{OperationBytes, Result};

mod value;
pub use value::Value;

mod map;
pub use map::Map;

mod vec;
pub use vec::Vec;

pub trait Model: Default + Debug {
    /// Get operations for this value.
    ///
    /// Don't forget clean this value to default.
    fn operations(&mut self) -> Result<alloc_vec<(alloc_vec<u8>, OperationBytes)>>;

    /// Define this type's code.
    fn type_code(&self) -> u32;

    /// Merge other value.
    fn merge(&mut self, other: Self);
}
