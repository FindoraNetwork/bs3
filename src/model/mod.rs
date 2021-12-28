//!
//! Trait Model constrains the behavior of the cache layer
//!

mod value;
use core::fmt::Debug;

pub use value::Value;

mod map;
pub use map::Map;

mod vec;
pub use vec::Vec;

// mod doublekey_map;
// pub use doublekey_map::DoubleKeyMap;

mod model;
pub use model::Model;

use crate::prelude::{FromBytes, ToBytes};

pub trait KeyT: Clone + PartialEq + Eq + FromBytes + ToBytes + PartialOrd + Ord + Debug {}

pub trait ValueT: Clone + Debug + FromBytes + ToBytes {}
