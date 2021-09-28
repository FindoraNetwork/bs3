//!
//! Storage layer implemented using sled
//!

use alloc::{boxed::Box, vec::Vec};
use core::ops::Bound::Included;
use sled::{Db, Iter, Tree};

use crate::{CowBytes, Error, Result};

use super::Store;
use core::ops::{Bound, RangeBounds};
use alloc::string::ToString;

///
/// use sled tree
pub struct SledBackend {
    tree: Tree,
}

fn e(e: sled::Error) -> Error {
    Error::StoreError(Box::new(e))
}

///
/// create temp dir
/// like
///     /tmp/bs3_tmp_2234422334
///
pub fn tmp_dir() -> std::path::PathBuf {
    let base_dir = std::env::temp_dir();
    let name = std::format!("{}_{}", "bs3_tmp", rand::random::<u64>());
    let path = base_dir.join(name);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir(&path).unwrap();
    path
}

/// create sled db
/// feat Compression
pub fn sled_db_open(path: Option<&str>) -> Result<sled::Db> {

    let mut is_tmp = false;

    let path = if let Some(path) = path {
        path.to_string()
    } else {
        let path = tmp_dir().to_str().unwrap().to_string();
        is_tmp = true;
        path
    };

    let mut cfg = sled::Config::default()
        .path(path)
        .mode(sled::Mode::HighThroughput)
        .cache_capacity(20_000_000)
        .flush_every_ms(Some(3000));

    if is_tmp {
        cfg = cfg.temporary(true);
    }

    #[cfg(feature = "compress")]
    let cfg = cfg.use_compression(true).compression_factor(15);

    Ok(cfg.open()?)
}

impl SledBackend {
    /// create tree
    pub fn open_tree(db: &Db, namespace: &str) -> Result<Self> {
        let tree = db.open_tree(namespace).map_err(e)?;
        Ok(Self { tree })
    }

    /// get
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.tree.get(key) {
            Ok(Some(v)) => Ok(Some(v.to_vec())),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::StoreError(Box::new(e))),
        }
    }
}

/// SledRange used to host the sled iter
pub struct SledRange<'a> {
    pub v: Iter,
    _s: &'a str,
}

/// impl Iterator
impl<'a> Iterator for SledRange<'a> {
    type Item = (CowBytes<'a>, CowBytes<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.v
            .next()
            .and_then(|item| item.ok())
            .map(|(key, val)| (CowBytes::Owned(key.to_vec()), CowBytes::Owned(val.to_vec())))
    }
}

/// impl DoubleEndedIterator
impl<'a> DoubleEndedIterator for SledRange<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.v
            .next_back()
            .and_then(|item| item.ok())
            .map(|(key, val)| (CowBytes::Owned(key.to_vec()), CowBytes::Owned(val.to_vec())))
    }
}

/// Searcher,Given start and end values
pub struct SledRangeBounds {
    begin_key: Vec<u8>,
    end_key: Vec<u8>,
}

/// impl RangeBounds,converts start and end values
impl RangeBounds<Vec<u8>> for SledRangeBounds {
    fn start_bound(&self) -> Bound<&Vec<u8>> {
        Included(self.begin_key.as_ref())
    }

    fn end_bound(&self) -> Bound<&Vec<u8>> {
        Included(self.end_key.as_ref())
    }
}

/// impl store
impl Store for SledBackend {
    type Range<'a> = SledRange<'a>;

    /// Search Scope
    fn range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<Self::Range<'_>> {
        let r = SledRangeBounds {
            begin_key: begin_key.to_vec(),
            end_key: end_key.to_vec(),
        };
        Ok(SledRange {
            v: self.tree.range(r),
            _s: "",
        })
    }

    /// Batch insert
    fn execute(&mut self, batch: Vec<(Vec<u8>, Vec<u8>)>) -> Result<()> {
        let inner = &mut self.tree;
        log::debug!("Write {} record", batch.len());
        for (key, value) in batch {
            log::debug!(target: "bs3-sled", "Insert key is : {:?}", key);
            log::debug!(target: "bs3-sled", "Insert value is : {:?}", value);
            let _ = inner.insert(key, value);
        }
        Ok(())
    }
}

// #[test]
// fn test(){
//     let path = "/tmp/bs3_sled_test/01";
//     let namespace = "test_tree";
//
//     let db = sled_db_open(path,false).unwrap();
//     let sb = SledBackend::open_tree(&db,namespace).unwrap();
//
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
