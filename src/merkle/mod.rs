use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{Result, Store};

pub mod empty;

pub trait Merkle: Default {
    type Digest: Digest;

    fn insert<S: Store>(&mut self, store: &mut S, batch: &[(Vec<u8>, Vec<u8>)]) -> Result<()>;

    fn root<S: Store>(&self, store: &S) -> Result<Output<Self::Digest>>;
}
