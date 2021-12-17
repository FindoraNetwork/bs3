//!
//! Transaction Middleware
//!

use crate::{backend::Store, merkle::Merkle, model::Model, SnapshotableStorage};

pub struct Transaction<'a, S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    pub store: &'a SnapshotableStorage<S, M, V>,
    pub value: V,
}

impl<'a, S, M, V> Clone for Transaction<'a, S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    fn clone(&self) -> Self {
        Self {
            store: self.store,
            value: self.value.clone(),
        }
    }
}
impl<'a, S, M, V> Transaction<'a, S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    pub fn new(store: &'a SnapshotableStorage<S, M, V>) -> Self {
        Transaction {
            store,
            value: V::default(),
        }
    }
}
