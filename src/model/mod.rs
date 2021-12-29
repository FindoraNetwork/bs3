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

pub trait KeyType: PartialEq + Eq + Ord + PartialOrd + ValueType {}
pub trait ValueType: Clone + Debug + Serialize + for<'de> Deserialize<'de> {}

macro_rules! impl_key_type {
    ( $($t:ty),* )=> {
        $(impl KeyType for $t {})*
    };
}

macro_rules! impl_value_type {
    ( $($t:ty),* )=> {
        $(impl ValueType for $t {})*
    };
}

impl_key_type! {u8, u16, u32, u64, i8, i16, i32, i64, String}
impl_value_type! {u8, u16, u32, u64, i8, i16, i32, i64, String}

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
