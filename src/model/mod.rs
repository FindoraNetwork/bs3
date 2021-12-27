//!
//! Trait Model constrains the behavior of the cache layer
//!

use core::fmt::Debug;

use alloc::string::String;
use alloc::vec::Vec as alloc_vec;

use crate::{OperationBytes, Result};

mod value;
pub use value::Value;

mod map;
pub use map::Map;

mod vec;
pub use vec::{Vec, INDEX_VEC_LEN};

mod doublekey_map;
pub use doublekey_map::DoubleKeyMap;

use serde::{Deserialize, Serialize};

pub trait KType:
    Clone + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> + Ord + PartialOrd + Debug
{
}
pub trait ValueType: Clone + Debug + Serialize + for<'de> Deserialize<'de> {}

macro_rules! impl_value_type {
    ( $($t:ty),* )=> {
        $(impl ValueType for $t {})*
    };
}

impl_value_type! {String, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize}

pub trait Model: Default + Debug + Clone {
    /// Get operations for this value.
    ///
    /// Don't forget clean this value to default.
    fn operations(&mut self) -> Result<alloc_vec<(alloc_vec<u8>, OperationBytes)>>;

    /// Define this type's code.
    fn type_code(&self) -> u32;

    /// Merge other value.
    fn merge(&mut self, other: Self);
}
