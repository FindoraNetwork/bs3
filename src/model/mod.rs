use core::fmt::Debug;

use alloc::vec::Vec;

use crate::{OperationBytes, Result, SnapshotableStorage, Store};

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

    /// Define this type's code.
    fn type_code(&self) -> u32;

    /// Merge other value.
    fn merge(&mut self, other: Self);
}

// pub struct Guard<'a, S, M>
// where
//     S: Store,
//     M: Model,
// {
//     pub value: &'a M,
//     pub store: &'a SnapshotableStorage<S, M>,
// }
