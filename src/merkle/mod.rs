use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{OperationBytes, Result, Store};

pub mod append_only;
pub mod empty;
pub mod sparse_merkle_tree;
mod utils;
mod value;

pub use utils::min;

pub trait Merkle: Default + Clone {
    type Digest: Digest;

    fn rollback(&mut self, target_height: i64) -> Result<()>;

    fn new(namespace: &str, height: i64) -> Self;

    fn insert<S: Store>(
        &mut self,
        store: &mut S,
        batch: &[(Vec<u8>, OperationBytes)],
    ) -> Result<()>;

    fn root<S: Store>(&self, store: &S) -> Result<Output<Self::Digest>>;
}
