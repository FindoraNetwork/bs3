use core::fmt::Debug;

use crate::{OperationBytes, Result};

use alloc::vec::Vec;

pub trait Model: Default + Debug + Clone {
    /// Get operations for this value.
    ///
    /// Don't forget clean this value to default.
    fn operations(&mut self) -> Result<Vec<(Vec<u8>, OperationBytes)>>;

    /// Define this type's code.
    fn type_code(&self) -> u32;

    /// Merge other value.
    fn merge(&mut self, other: Self);
}
