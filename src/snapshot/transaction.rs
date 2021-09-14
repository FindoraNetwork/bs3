//!
//! Transaction Middleware
//!

use crate::{backend::Store, model::Model, SnapshotableStorage};

pub struct Transaction<'a, S, M>
where
    S: Store,
    M: Model,
{
    pub store: &'a SnapshotableStorage<S, M>,
    pub value: M,
}

impl<'a, S, M> Transaction<'a, S, M>
where
    S: Store,
    M: Model,
{
    pub fn new(store: &'a SnapshotableStorage<S, M>) -> Self {
        Transaction {
            store,
            value: M::default(),
        }
    }
}
