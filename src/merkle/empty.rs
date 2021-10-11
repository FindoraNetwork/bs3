use core::marker::PhantomData;

use alloc::vec::Vec;
use digest::{Digest, Output};

use crate::{OperationBytes, Result, Store};

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

    fn insert<S: Store>(&mut self, _prev_key: Vec<u8>, _cur_key:Vec<u8>, _store: &mut S, _batch: &[(Vec<u8>, OperationBytes)]) -> Result<()> {
        Ok(())
    }

    fn root<S: Store>(&self, _key:Vec<u8>, _store: &S) -> Result<Option<Output<D>>> {
        Ok(None)
    }
}
