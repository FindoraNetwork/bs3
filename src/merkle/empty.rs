use core::marker::PhantomData;

use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{OperationBytes, Result, Store};

use super::Merkle;

#[derive(Clone)]
pub struct EmptyMerkle<D: Digest + Clone> {
    marker: PhantomData<D>,
}

impl<D: Digest + Clone> Default for EmptyMerkle<D> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<D: Digest + Clone> Merkle for EmptyMerkle<D> {
    type Digest = D;

    fn rollback(&mut self, _target_height: i64) -> Result<()> {
        Ok(())
    }

    fn new(_namespace: &str, _height: i64) -> Self {
        EmptyMerkle::default()
    }

    fn insert<S: Store>(
        &mut self,
        _store: &mut S,
        _batch: &[(Vec<u8>, OperationBytes)],
    ) -> Result<()> {
        Ok(())
    }

    fn root<S: Store>(&self, _store: &S) -> Result<Output<D>> {
        Ok(Default::default())
    }
}
