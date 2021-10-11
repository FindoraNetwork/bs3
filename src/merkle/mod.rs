use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{OperationBytes, Result, Store};

pub mod empty;
pub mod append_only;
pub mod sparse_merkle_tree;
mod utils;

pub use utils::min;

pub trait Merkle: Default {
    type Digest: Digest;

    fn insert<S: Store>(&mut self, prev_key: Vec<u8>, cur_key:Vec<u8>, store: &mut S, batch: &[(Vec<u8>, OperationBytes)]) -> Result<()>;

    fn root<S: Store>(&self, key:Vec<u8>, store: &S) -> Result<Option<Output<Self::Digest>>>;
}
