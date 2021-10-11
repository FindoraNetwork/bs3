//!
//! Transaction Middleware
//!

use crate::{SnapshotableStorage, backend::Store, merkle::Merkle, model::Model};

pub struct Transaction<'a, S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    pub store: &'a SnapshotableStorage<S, M, V>,
    pub value: V,
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
