use core::mem;

use crate::{
    backend::{Operation, Store},
    prelude::TreeMut,
    Error, Result,
};
use alloc::{collections::BTreeMap, vec::Vec};
use digest::{Digest, Output};

use self::value::{StoreHeight, ToStoreBytes};

// mod direct;
mod utils;
mod value;

pub struct SnapshotableStorage<'a, D: Digest> {
    store: &'a dyn Store,
    height: u64,
    cache: BTreeMap<Output<D>, Operation>,
    namespace: &'a str,
}

/// Methods for create storage.
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    pub fn new(store: &'a impl Store) -> Self {
        Self::new_with_namespace(store, "")
    }

    pub fn new_with_height(store: &'a impl Store, height: u64) -> Result<Self> {
        let mut s = Self::new(store);
        s.rollback(height)?;
        Ok(s)
    }

    pub fn new_with_namespace(store: &'a impl Store, namespace: &'a str) -> Self {
        Self {
            store: store as &'a dyn Store,
            height: 0,
            cache: BTreeMap::new(),
            namespace,
        }
    }

    pub fn new_with_height_namespace(
        store: &'a impl Store,
        height: u64,
        namespace: &'a str,
    ) -> Result<Self> {
        let mut s = Self {
            store: store as &'a dyn Store,
            height,
            cache: BTreeMap::default(),
            namespace,
        };
        s.rollback(height)?;
        Ok(s)
    }
}

/// Methods for snapshot.
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    /// rollback to point height, target_height must less than current height.
    pub fn rollback(&mut self, target_height: u64) -> Result<()> {
        if target_height > self.height {
            Err(Error::HeightError)
        } else {
            self.sync_height(target_height)
        }
    }

    /// Commit this snapshot.
    pub fn commit(&mut self) -> Result<u64> {
        let mut operations = Vec::new();

        let cache = mem::replace(&mut self.cache, BTreeMap::new());

        for (k, v) in cache {
            let key_bytes = utils::storage_key(self.namespace, &k, self.height);
            operations.push((key_bytes, v));
        }
        // incr current height

        self.sync_height(self.height + 1)?;

        Ok(self.height)
    }
}

/// Methods for internal helper
impl<'a, D: Digest> SnapshotableStorage<'a, D> {
    pub(crate) fn sync_height(&mut self, target_height: u64) -> Result<()> {
        self.height = target_height;

        let mut operations = Vec::new();
        let store_height = StoreHeight {
            height: self.height,
        };
        let height_key_bytes = utils::current_height_key(self.namespace);
        let height_value_bytes = Operation::Update(store_height.to_bytes()?);
        operations.push((height_key_bytes, height_value_bytes));
        self.store.execute(operations)?;

        Ok(())
    }

    //     pub(crate) fn direct_raw_get(&self, key: &Output<D>) -> Result<Option<Vec<u8>>> {
    // let key = storage_key(self.namespace, key, self.height);
    // let value = self.store.get_lt(&key)?;
    // if let Some(bytes) = value {
    //     Ok(Some(bytes))
    // } else {
    //     Ok(None)
    // }
    // }
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

impl<'a, D: Digest> TreeMut<D> for SnapshotableStorage<'a, D> {
    fn get_mut(&mut self, _key: &Output<D>) -> Result<Option<&mut [u8]>> {
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
