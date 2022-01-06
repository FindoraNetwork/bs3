use alloc::vec::Vec;

use crate::{
    model::Model,
    prelude::{FromBytes, ToBytes},
    types::{BranchName, OperationBytes, StoreKey, StoreValue, VersionName},
    utils, Result, Store,
};

pub struct Storage<S: Store, V: Model> {
    pub(crate) store: S,
    pub value: V,
    pub(crate) version: VersionName,
    pub(crate) branch: BranchName,
    pub(crate) version_id: u64,
    pub(crate) branch_id: u64,
    pub(crate) allocable_key_id: u64,
}

/// Storage.
///
/// Data in storage:
/// ```
/// DataStore      d:bid-s:kid-vid -> {Update(data), Delete}
/// BranchMapStore m:bid-s:vid -> merge_from(u64)
/// VersionStore   v:bid-n:version -> vid(u64)
/// KeyStore       k:bid-n:key -> kid(u64)
/// BranchStore    b:branch -> <parent_branch(u64), allocable_key_id(u64), bid(u64), version_id(u64),
/// allocable_key_id(u64)>
/// ```
///
/// ## Logic
///
/// ### GetValueByVseq(bseq, kseq, vseq)
///
/// 1. SGetLt `d:bseq-k:kseq-vseq` -> data
///     - If data == Some(Update(d)) -> return Some(d),
///     - If data == Some(Delete) -> return None,
///     - If data == None -> return None,
///     - If data == Some(Ref(_bseq, _vseq)) -> return call GetValueByVseq(_bseq, kseq, _vseq),
///
///
/// ### GetValueByKey(self, key)
///
/// 1. SGet `k:self.bseq-k:key` -> kseq.
/// 2. return call GetValueByVseq(self.bseq, kseq, self.bseq)
///
/// ### StoreKeyValue(self, key, value: StoreData)
///
/// 1. Get `k:self.bseq-key` -> kseq
///     - If kseq == None -> `id = self.kseq + 1`
///         - Store (`k:self.bseq-n:key`, `id`).
///     - If kseq == Some(id) -> id
/// 2. Store `k:self.bseq-s:kseq-self.vseq` => `value`
///
/// ### NewBranch(store, branch_name, version_name, value)
/// 1.
impl<S, V> Storage<S, V>
where
    S: Store,
    V: Model,
{
    pub fn get_by_id(
        &self,
        branch_id: u64,
        key_id: u64,
        version_id: u64,
    ) -> Result<Option<Vec<u8>>> {
        let begin_key = utils::data_store_key(branch_id, key_id, 0);
        let end_key = utils::data_store_key(branch_id, key_id, version_id);

        let res = if let Some(result) = self.store.get_in(&begin_key, &end_key)? {
            let sv = StoreValue::from_bytes(&result)?;

            match sv {
                StoreValue::Update(data) => Some(data),
                StoreValue::Delete => None,
                StoreValue::Ref {
                    branch_id: bid,
                    version_id: vid,
                } => self.get_by_id(bid, key_id, vid)?,
            }
        } else {
            None
        };

        Ok(res)
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let key_key = utils::key_store_key(self.branch_id, key)?;

        let res = if let Some(key_id) = self.store.get(&key_key)? {
            let key_store = StoreKey::from_bytes(&key_id)?;

            self.get_by_id(self.branch_id, key_store.key_id, self.version_id)?
        } else {
            None
        };

        Ok(res)
    }

    pub fn store_batch(&mut self, kvs: Vec<(Vec<u8>, StoreValue)>) -> Result<()> {
        for (key, value) in kvs {
            let key_key = utils::key_store_key(self.branch_id, &key)?;

            let key_id = if let Some(id) = self.store.get(&key_key)? {
                let key_store = StoreKey::from_bytes(&id)?;
                key_store.key_id
            } else {
                self.allocable_key_id += 1;
                self.allocable_key_id
            };

            let data_key = utils::data_store_key(self.branch_id, key_id, self.version_id);
            self.store.insert(data_key, value.to_bytes()?)?;
        }

        Ok(())
    }

    pub fn new(
        store: S,
        branch: BranchName,
        branch_id: u64,
        version: VersionName,
        value: V,
    ) -> Result<Self> {
        let mut value = value;

        let operations = value
            .operations()?;

        let mut kvs = Vec::new();
        for (k, v) in operations {
            kvs.push((k, v.into()))
        }

        let mut s = Self {
            store,
            branch: branch.clone(),
            version: version.clone(),
            value,
            allocable_key_id: 0,
            branch_id,
            version_id: 0,
        };

        s.store_batch(kvs)?;

        let version_key = utils::version_store_key(branch_id, &version)?;

        let branch_key = utils::branch_store_key(&branch)?;

        Ok(s)
    }

    pub fn version(&self) -> &VersionName {
        &self.version
    }

    pub fn branch(&self) -> &BranchName {
        &self.branch
    }

    //     pub fn root(&self) -> Result<Output<M::Digest>> {
    // Ok(self.merkle.root())
    //     }

    pub fn commit(&mut self, version: VersionName) -> Result<()> {
        Ok(())
    }

    pub fn rollback(&mut self, version: &VersionName) -> Result<()> {
        Ok(())
    }

    pub fn revert(&mut self) -> Result<()> {
        Ok(())
    }

    //     pub fn fork(&self, branch: BranchName) -> Result<Self> {
    // let branch = Self {
    //     store: self.store.clone(),
    //     value: self.value.clone(),
    //     version: self.version.clone(),
    //     branch,
    // };
    //
    // Ok(branch)
    //     }

    pub fn merge(&self, branch: Self) -> Result<()> {
        Ok(())
    }
}
