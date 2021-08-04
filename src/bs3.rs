use core::marker::PhantomData;

use digest::Digest;

use crate::{backend::Store, Transaction};

pub struct Bs3<D: Digest, S: Store> {
    marker: PhantomData<D>,
    store: S,
}

impl<D: Digest, S: Store> Bs3<D, S> {
    pub fn transaction(&self) -> Transaction<'_, D, S> {
        Transaction::new(&self.store)
    }
}
