use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{OperationBytes, Result, Store};

pub mod empty;

pub trait Merkle: Default {
    type Digest: Digest;

    fn insert<S: Store>(&mut self, store: &mut S, batch: &[(Vec<u8>, OperationBytes)]) -> Result<()>;

    fn root<S: Store>(&self, store: &S) -> Result<Output<Self::Digest>>;
}
