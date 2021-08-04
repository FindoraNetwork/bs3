use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::backend::Store;

pub struct Transaction<'a, D: Digest, S: Store> {
    pub(crate) store: &'a S,
    pub(crate) cache: BTreeMap<Output<D>, Vec<u8>>,
}

impl<'a, D, S> Transaction<'a, D, S>
where
    D: Digest,
    S: Store,
{
    pub(crate) fn new(store: &'a S) -> Self {
        Transaction {
            store,
            cache: BTreeMap::new(),
        }
    }
}
