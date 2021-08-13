use alloc::{borrow::Cow, vec::Vec};
use digest::{Digest, Output};

use crate::Result;

pub trait Tree<D: Digest> {
    /// Get value by key in tree.
    ///
    /// Note: use CoW to avoid use refcell.
    fn get(&self, key: &Output<D>) -> Result<Option<Cow<'_, [u8]>>>;
}

pub trait TreeMut<D: Digest> {
    /// Get value mut.
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>>;

    /// Insert value.
    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>>;

    /// Remove value.
    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>>;
}
