use crate::{backend::Store, model::Model, SnapshotableStorage};

pub struct Transaction<'a, S, M>
where
    S: Store,
    M: Model,
{
    pub(crate) store: &'a mut SnapshotableStorage<S, M>,
    pub(crate) value: M,
}

impl<'a, S, M> Transaction<'a, S, M>
where
    S: Store,
    M: Model,
{
    pub(crate) fn new(store: &'a mut SnapshotableStorage<S, M>) -> Self {
        Transaction {
            store,
            value: M::default(),
        }
    }
}
