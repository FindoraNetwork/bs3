use core::cell::RefCell;

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use sled::{Db, Tree};

use crate::{Error, Result};

// use super::Store;

pub struct SledBackend {
    cache: Arc<BTreeMap<Vec<u8>, Vec<u8>>>,
    rc_ptr: RefCell<Arc<BTreeMap<Vec<u8>, Vec<u8>>>>,
    tree: Tree,
}

fn e(e: sled::Error) -> Error {
    Error::StoreError(Box::new(e))
}

impl SledBackend {
    pub fn open_tree(db: &Db, namespace: &str) -> Result<Self> {
        let tree = db.open_tree(namespace).map_err(e)?;
        let cache = Arc::new(BTreeMap::new());
        let rc_ptr = RefCell::new(cache.clone());
        Ok(Self {
            cache,
            rc_ptr,
            tree,
        })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
        Ok(match self.cache.get(key) {
            Some(v) => Some(v.as_slice()),
            None => match self.tree.get(key).map_err(e)? {
                Some(v) => {
                    let mut rc_ptr = self.rc_ptr.borrow_mut().clone();
                    let cache_mut = Arc::get_mut(&mut rc_ptr).unwrap();
                    cache_mut.insert(Vec::from(key), v.to_vec());
                    None
                }
                None => None,
            },
        })
    }

    pub fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use std::{println, sync::Mutex};

    use alloc::{collections::BTreeMap, sync::Weak, sync::Arc, vec, vec::Vec};

    use crate::Result;

    pub type MemTree = BTreeMap<Vec<u8>, Vec<u8>>;

    pub struct TestBackend {
        pub cache: Arc<MemTree>,
        rc_ptr: RefCell<Arc<MemTree>>,
    }

    impl TestBackend {
        pub fn new() -> Self {
            let cache = Arc::new(BTreeMap::new());

            let rc_ptr = RefCell::new(cache.clone());
            Self { cache, rc_ptr }
        }

        pub fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
            let value = vec![1, 2, 3];

            let has_key = self.cache.contains_key(key);
            println!("size?: {}", Arc::strong_count(&self.cache));

            if !has_key {
                let mut rc_ptr = self.rc_ptr.borrow_mut();
                println!("size?: {}", Arc::strong_count(&rc_ptr));
                let cache_mut = Arc::get_mut(&mut rc_ptr).unwrap();
                cache_mut.insert(Vec::from(key), value);
            }

            // let cache = self.cache.upgrade().unwrap();
            // Ok(cache.get(key).map(|v| v.as_slice()))
            Ok(None)
        }
    }

    #[test]
    fn test_arc() {
        let testing = TestBackend::new();

        let key = vec![1, 2, 3];

        println!("result is: {:?}", testing.get(&key));

        println!("inner is: {:?}", testing.cache);

        println!("result is: {:?}", testing.get(&key));
    }
}
