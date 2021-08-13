use crate::{
    backend::{Operation, Store},
    prelude::{Tree, TreeMut},
    Result,
};
use alloc::{borrow::Cow, collections::BTreeMap, format, vec::Vec};
use digest::{Digest, Output};

use self::utils::storage_key;

// mod direct;
mod utils;

pub struct SnapshotedStorage<'a, D: Digest> {
    store: &'a dyn Store,
    height: u64,
    cache: BTreeMap<Output<D>, Operation>,
    namespace: &'a str,
}

/// Methods for create storage.
impl<'a, D: Digest> SnapshotedStorage<'a, D> {
    pub fn new_with_height_namespace(
        store: &'a impl Store,
        height: u64,
        namespace: &'a str,
    ) -> Result<Self> {
        let mut s = SnapshotedStorage {
            store: store as &'a dyn Store,
            height,
            cache: BTreeMap::default(),
            namespace,
        };
        s.rollback(height)?;
        Ok(s)
    }

    pub fn new(store: &'a impl Store) -> Self {
        Self::new_with_namespace(store, "")
    }

    pub fn new_with_namespace(store: &'a impl Store, namespace: &'a str) -> Self {
        SnapshotedStorage {
            store: store as &'a dyn Store,
            height: 0,
            cache: BTreeMap::new(),
            namespace,
        }
    }

    pub fn new_with_height(store: &'a impl Store, height: u64) -> Result<Self> {
        let mut s = Self::new(store);
        s.rollback(height)?;
        Ok(s)
    }
}

/// Methods for snapshot.
impl<'a, D: Digest> SnapshotedStorage<'a, D> {
    pub fn rollback(&mut self, target_height: u64) -> Result<()> {
        Ok(())
    }

    pub fn commit(&mut self) -> Result<u64> {
        let operations = Vec::new();
        for (k, v) in &self.cache {
            let key = utils::storage_key(self.namespace, &k, self.height);
            // let key =
        }
        // operations.push(utils::current_height_key(self.namespace), );
        self.store.execute(operations)?;
        self.cache.clear();
        self.height += 1;
        Ok(self.height)
    }
}

/// Methods for internal helper
impl<'a, D: Digest> SnapshotedStorage<'a, D> {
    pub(crate) fn direct_raw_get(&self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        let key = storage_key(self.namespace, key, self.height);
        let value = self.store.get_lt(&key)?;
        if let Some(bytes) = value {
            Ok(Some(bytes))
        } else {
            Ok(None)
        }
    }
}


// impl<'a, D: Digest> Tree<D> for SnapshotedStorage<'a, D> {
// fn get(&self, key: &Output<D>) -> Result<Option<Cow<'_, [u8]>>> {
//     if let Some(r) = self.cache.get(key) {
//         Ok(Some(Cow::Borrowed(r)))
//     } else {
//         Ok(None)
//     }
// }
// }

impl<'a, D: Digest> TreeMut<D> for SnapshotedStorage<'a, D> {
    fn get_mut(&mut self, key: &Output<D>) -> Result<Option<&mut [u8]>> {
        Ok(None)
    }

    fn insert(&mut self, key: Output<D>, value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        if let Some(Operation::Update(v)) = self.cache.insert(key, Operation::Update(value)) {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
        if let Some(Operation::Update(v)) = self.cache.insert(key.clone(), Operation::Delete) {
            // insert into cache, if cache has this value, return this value.
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}
