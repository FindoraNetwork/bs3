use alloc::{string::String, vec::Vec};

use crate::{Error, Result, backend::Store, merkle::Merkle, model::Model, snapshot::StoreValue};

use super::{utils, value::StoreType, FromStoreBytes, StoreHeight, ToStoreBytes, Transaction};
use crate::snapshot::utils::storage_key;

/// Snapshotable Storage
pub struct SnapshotableStorage<S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    pub(crate) store: S,
    pub height: i64,
    pub(crate) value: V,
    pub(crate) namespace: String,
    pub(crate) merkle: M,
}

/// Methods for create storage.
impl<S, M, V> SnapshotableStorage<S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    /// Create a `SnapshotableStorage` from store with empty namespace.
    pub fn new(value: V, store: S) -> Result<Self> {
        Self::new_with_name(value, String::new(), store)
    }

    /// Create a `SnapshotableStorage` from store.
    pub fn new_with_name(value: V, name: String, store: S) -> Result<Self> {
        let mut s = Self {
            store,
            height: 0,
            namespace: name,
            value,
            merkle: M::default(),
        };

        if !s.init_or_load()? {
            // load height.

            log::debug!(
                "Load snapshotable store with namespace {}, at height: {}",
                s.namespace,
                s.height
            );
            s.height = s.read_height()?;
        }

        Ok(s)
    }

    /// Create a `SnapshotableStorage` from store for point height with empty namespace.
    ///
    /// If height is 0, equal to init a store.
    pub fn new_with_height(value: V, height: i64, store: S) -> Result<Self> {
        Self::new_with_height_namespace(value, height, String::new(), store)
    }

    /// Create a `SnapshotableStorage` from store for point height.
    ///
    /// If height is 0, equal to init a store.
    pub fn new_with_height_namespace(
        value: V,
        height: i64,
        namespace: String,
        store: S,
    ) -> Result<Self> {
        let mut s = Self {
            store: store,
            height,
            value,
            namespace,
            merkle: M::default(),
        };

        if height == 0 {
            s.init()?;
        } else {
            s.rollback(height)?;
        };

        Ok(s)
    }

    /// Init a new store.
    fn init(&mut self) -> Result<()> {
        let key = utils::type_key(&self.namespace);

        log::debug!(
            "Inital snapshotable store with namespace: {}",
            self.namespace
        );

        let store_type = StoreType {
            ty: self.value.type_code(),
        };
        let bytes = store_type.to_bytes()?;
        self.store.insert(key, bytes)?;
        self.write_height(0, None)?;
        Ok(())
    }

    /// Init or load a new store.
    fn init_or_load(&mut self) -> Result<bool> {
        let key = utils::type_key(&self.namespace);
        if let Some(bytes) = self.store.get_ge(&key)? {
            let store_type = StoreType::from_bytes(&bytes)?;
            match store_type.ty == self.value.type_code() {
                true => Ok(false),
                false => Err(Error::TypeMissMatch),
            }
        } else {
            self.init()?;
            Ok(true)
        }
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn get_with_height(&self, key: &str, height: i64) -> Result<Option<Vec<u8>>> {
        let begin_key = storage_key(&*self.namespace, key, 0);
        let end_key = storage_key(&*self.namespace, key, height);

        if let Some(v) = self.store.get_ge2((&begin_key, &end_key))? {
            Ok(Some(v.to_vec()))
        } else {
            Ok(None)
        }
    }
}

/// Methods for snapshot.
impl<S, M, V> SnapshotableStorage<S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    /// Read current height in store.
    fn read_height(&self) -> Result<i64> {
        let key = utils::current_height_key(&self.namespace);

        if let Some(bytes) = self.store.get_ge(&key)? {
            let store_height = StoreHeight::from_bytes(&bytes)?;
            Ok(store_height.height)
        } else {
            Ok(0)
        }
    }

    /// Force to write height in store.
    fn write_height(
        &mut self,
        target_height: i64,
        pre_commit: Option<Vec<(Vec<u8>, Vec<u8>)>>,
    ) -> Result<()> {
        log::debug!("Begin sync snapshot success in height: {}", self.height + 1);

        self.height = target_height;

        let mut operations = if let Some(ops) = pre_commit {
            ops
        } else {
            Vec::new()
        };

        let store_height = StoreHeight {
            height: self.height,
        };
        let height_key_bytes = utils::current_height_key(&self.namespace);
        let height_value_bytes = store_height.to_bytes()?;
        operations.push((height_key_bytes, height_value_bytes));
        self.store.execute(operations)?;

        Ok(())
    }

    /// rollback to point height, target_height must less than current height.
    pub fn rollback(&mut self, target_height: i64) -> Result<()> {
        if target_height > self.height {
            log::error!(
                "Target height {} must less than current height {}",
                target_height,
                self.height
            );
            Err(Error::HeightError)
        } else {
            self.write_height(target_height, None)
        }
    }

    pub(crate) fn storage_key(&self, key: &Vec<u8>) -> Vec<u8> {
        utils::storage_key(&self.namespace, &key, self.height)
    }

    pub(crate) fn storage_tuple_key(&self, key: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        (
            utils::storage_key(&self.namespace, &key, 0),
            utils::storage_key(&self.namespace, &key, self.height),
        )
    }

    /// Commit this snapshot.
    pub fn commit(&mut self) -> Result<i64> {
        let mut operations = Vec::new();

        let mut merkle_operations = Vec::new();

        log::debug!("Snapshot Cache: {:?}", self.value);

        for (k, v) in self.value.operations()? {
            let key_bytes = self.storage_key(&k);
            let store_value = StoreValue { operation: v.clone() };
            operations.push((key_bytes, store_value.to_bytes()?));
            merkle_operations.push((k, v));
        }

        // incr current height
        self.write_height(self.height + 1, Some(operations))?;

        log::debug!("Start Compute merkle");
        self.merkle.insert(&mut self.store, &merkle_operations)?;


        log::debug!("Sync snapshot success in height: {}", self.height);

        Ok(self.height)
    }

    pub fn root(&self) -> Result<Option<digest::Output<M::Digest>>> {
        self.merkle.root(&self.store)
    }
}

/// Methods for transaction
impl<S, M, V> SnapshotableStorage<S, M, V>
where
    S: Store,
    M: Merkle,
    V: Model,
{
    /// Generate transaction for this Bs3 db.
    pub fn transaction(&self) -> Transaction<'_, S, M, V> {
        Transaction::new(self)
    }

    /// Consume transaction to apply.
    pub fn execute(&mut self, val: V) {
        log::debug!("Transaction Cache: {:?}", val);
        self.value.merge(val)
    }
}

