use alloc::vec::Vec;
use digest::Output;

use crate::{
    merkle::Merkle,
    model::Model,
    types::{BranchName, VersionName},
    Store, Result,
};

pub struct Storage<S: Store, V: Model, M: Merkle> {
    pub(crate) store: S,
    pub value: V,
    pub(crate) merkle: M,
    pub(crate) version: VersionName,
    pub(crate) branch: BranchName,
}

impl<S, V, M> Storage<S, V, M>
where
    S: Store,
    V: Model,
    M: Merkle,
{
    pub fn new(store: S, merkle: M, value: V, version: VersionName, branch: BranchName) -> Self {
        Self {
            store,
            merkle,
            value,
            version,
            branch,
        }
    }

    pub fn version(&self) -> &VersionName {
        &self.version
    }

    pub fn branch(&self) -> &BranchName {
        &self.branch
    }

    pub fn root(&self) -> Result<Output<M::Digest>> {
        Ok(self.merkle.root())
    }

    pub fn commit(&mut self, version: VersionName) -> Result<()> {
        Ok(())
    }

    pub fn revert(&mut self, version: &VersionName) -> Result<()> {
        Ok(())
    }

    pub fn fork(&self, branch: BranchName) -> Result<Self> {
        let branch = Self {
            store: self.store.clone(),
            value: self.value.clone(),
            merkle: self.merkle.clone(),
            version: self.version.clone(),
            branch,
        };

        Ok(branch)
    }

    pub fn merge(&self, branch: Self) -> Result<()> {
        Ok(())
    }
}
