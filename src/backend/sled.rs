use alloc::{boxed::Box, sync::Arc, vec::Vec};
use sled::{Db, Tree};

use crate::{Error, Result};

use super::Store;

// use super::Store;

pub struct SledBackend {
    tree: Tree,
}

fn e(e: sled::Error) -> Error {
    Error::StoreError(Box::new(e))
}

impl SledBackend {
    pub fn open_tree(db: &Db, namespace: &str) -> Result<Self> {
        let tree = db.open_tree(namespace).map_err(e)?;
        Ok(Self { tree })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.tree.get(key) {
            Ok(Some(v)) => Ok(Some(v.to_vec())),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::StoreError(Box::new(e))),
        }
    }

    pub fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        Ok(())
    }
}

// impl Store for SledBackend {
// type Range = sled::Iter;
//
// fn range(&self, begin_key: Vec<u8>, end_key: Vec<u8>) -> Result<Self::Range> {
//    // match self
// }
// }

// #[cfg(test)]
// mod tests {
//     use core::cell::RefCell;
//     use std::println;
//
//     use alloc::{collections::BTreeMap, sync::Arc, vec, vec::Vec};
//
//     use crate::Result;
//
//     pub type MemTree = BTreeMap<Vec<u8>, Vec<u8>>;
//
//     pub struct TestBackend {
//         pub cache: Arc<MemTree>,
//         rc_ptr: RefCell<Arc<MemTree>>,
//     }
//
//     impl TestBackend {
//         pub fn new() -> Self {
//             let cache = Arc::new(BTreeMap::new());
//
//             let rc_ptr = RefCell::new(cache.clone());
//             Self { cache, rc_ptr }
//         }
//
//         pub fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
//             let value = vec![1, 2, 3];
//
//             let has_key = self.cache.contains_key(key);
//             println!("size?: {}", Arc::strong_count(&self.cache));
//
//             if !has_key {
//                 let mut rc_ptr = self.rc_ptr.borrow_mut();
//                 println!("size?: {}", Arc::strong_count(&rc_ptr));
//                 let cache_mut = Arc::get_mut(&mut rc_ptr).unwrap();
//                 cache_mut.insert(Vec::from(key), value);
//             }
//
//             // let cache = self.cache.upgrade().unwrap();
//             // Ok(cache.get(key).map(|v| v.as_slice()))
//             Ok(None)
//         }
//     }
//
//     #[test]
//     fn test_arc() {
//         let testing = TestBackend::new();
//
//         let key = vec![1, 2, 3];
//
//         println!("result is: {:?}", testing.get(&key));
//
//         println!("inner is: {:?}", testing.cache);
//
//         println!("result is: {:?}", testing.get(&key));
//     }
// }
