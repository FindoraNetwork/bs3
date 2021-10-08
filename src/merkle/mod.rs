use digest::{Digest, Output};

use crate::Result;

pub mod empty;

pub trait Merkle {
    type Digest: Digest;

    fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<()>;

    fn root(&mut self) -> Result<Output<Self::Digest>>;
}



