use crate::Result;
use alloc::vec::Vec;
use digest::{Digest, Output};

// /// Get result.
// pub enum Bytes<'a> {
//     Owned(Vec<u8>),
//     Borrowed(&'a [u8]),
// }
//
// impl<'a> Deref for Bytes<'a> {
//     type Target = [u8];
//
//     fn deref(&self) -> &Self::Target {
//         match self {
//             Self::Owned(v) => v.as_slice(),
//             Self::Borrowed(v) => *v,
//         }
//     }
// }
//
pub trait Tree<D: Digest> {
    /// Get value by key in tree.
    ///
    /// Note: use CoW to avoid use refcell.
    fn get(&self, key: &Output<D>) -> Result<Option<&[u8]>>;
}

pub trait TreeMut<D: Digest> {
    /// Get value mut.
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>>;

    /// Insert value.
    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>>;

    /// Remove value.
    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>>;
}
