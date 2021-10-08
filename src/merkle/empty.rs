use core::marker::PhantomData;

use digest::{Digest, Output};

use crate::Result;

use super::Merkle;

pub struct EmptyMerkle<D: Digest> {
    marker: PhantomData<D>,
}

impl<D: Digest> Merkle for EmptyMerkle<D> {
    type Digest = D;

    fn insert(&mut self, _key: &[u8], _value: &[u8]) -> Result<()> {
        Ok(())
    }

    fn root(&mut self) -> Result<Output<D>> {
        Ok(Default::default())
    }
}

