//!
//! Transaction Middleware
//!

use crate::{backend::Store, merkle::Merkle, model::Model, SnapshotableStorage};

pub trait Forkable {
    type Cache;

    fn cache(self) -> Self::Cache;

    fn merge(&mut self, v: Self::Cache);
}

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

impl<'a, S, M, V> Forkable for Transaction<'a, S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    type Cache = V;

    fn cache(self) -> Self::Cache {
        self.value
    }

    fn merge(&mut self, v: Self::Cache) {
        self.execute(v)
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

    pub fn execute(&mut self, val: V) {
        log::debug!("Transaction Cache: {:?}", val);
        self.value.merge(val)
    }
}
