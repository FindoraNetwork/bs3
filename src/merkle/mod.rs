use digest::{Digest, Output};

use crate::Result;

pub mod empty;

pub trait Merkle: Default {
    type Digest: Digest;

    fn insert(&mut self, batch: &[(&[u8], &[u8])]) -> Result<()>;

    fn root(&mut self) -> Result<Output<Self::Digest>>;
}
