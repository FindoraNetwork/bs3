use alloc::{collections::BTreeMap, vec::Vec};

// use crate::Result;
//
// use super::Store;
//
pub struct MemoryBackend {
    store: BTreeMap<Vec<u8>, Vec<u8>>,
}

// impl Store for MemoryBackend {
//     fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
//         Ok(match self.store.get(key) {
//             Some(v) => Some(v.as_slice()),
//             None => None
//         })
//     }
//
//     fn get_lt(&self, key: &[u8]) -> Result<Option<&[u8]>> {
//         Ok(match self.store.is_greater_or_equal_private)
//     }
// }
//
