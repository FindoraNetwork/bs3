use digest::{Digest, Output};

use crate::{Result, bytes_ref::BytesRef, snapshot::utils};

use super::Store;

/// Get value in target height directly.
pub fn raw_get_lt<'a, D: Digest, S: Store>(store: &'a S, namespace: &str, key: &Output<D>, height: u64) -> Result<Option<BytesRef<'a>>> {
    let end_key = utils::storage_key(namespace, key, height);
    let begin_key = utils::storage_key(namespace, key, 0);
    let mut value = store.range(begin_key, end_key)?;
    Ok(match value.next() {
        Some((_, v)) => Some(v),
        None => None,
    })
}

