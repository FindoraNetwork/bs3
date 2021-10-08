use core::marker::PhantomData;

use digest::{Digest, Output};

use crate::Result;

use super::Merkle;

pub struct EmptyMerkle<D: Digest> {
    marker: PhantomData<D>,
}

impl<D: Digest> Default for EmptyMerkle<D> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<D: Digest> Merkle for EmptyMerkle<D> {
    type Digest = D;

    fn insert(&mut self, _batch: &[(&[u8], &[u8])]) -> Result<()> {
        Ok(())
    }

    fn root(&mut self) -> Result<Output<D>> {
        Ok(Default::default())
    }
}
