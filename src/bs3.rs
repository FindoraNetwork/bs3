use core::marker::PhantomData;

use digest::Digest;

use crate::{backend::Store, Result, Transaction};

pub struct Bs3<D: Digest, S: Store> {
    marker: PhantomData<D>,
    store: S,
}

impl<D: Digest, S: Store> Bs3<D, S> {
    /// Generate transaction for this Bs3 db.
    ///
    /// Use immutable reference let you can start multiple transactions.
    pub fn transaction(&self) -> Transaction<'_, D, S> {
        Transaction::new(&self.store)
    }

    /// Consume transaction to apply.
    pub fn execute(&mut self, tx: Transaction<'_, D, S>) -> Result<()> {
        self.store.execute(tx.to_operations())
    }
}
