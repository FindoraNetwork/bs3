use core::marker::PhantomData;

use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{Result, Store};

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

    fn insert<S: Store>(&mut self, _store: &mut S, _batch: &[(Vec<u8>, Vec<u8>)]) -> Result<()> {
        Ok(())
    }

    fn root<S: Store>(&self, _store: &S) -> Result<Output<D>> {
        Ok(Default::default())
    }
}
