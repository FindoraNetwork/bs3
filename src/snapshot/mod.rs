use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use crate::backend::Store;

pub struct SnapshotedStorage<'a, D: Digest> {
    store: &'a dyn Store,
    height: u32,
    cache: BTreeMap<Output<D>, Vec<u8>>,
}
