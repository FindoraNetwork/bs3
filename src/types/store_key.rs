use alloc::vec::Vec;

use crate::{
    prelude::{FromBytes, ToBytes},
    Result,
};

#[derive(Debug, Clone)]
pub struct StoreKey {
    pub key_id: u64,
}

impl ToBytes for StoreKey {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.key_id.to_be_bytes().to_vec())
    }
}

impl FromBytes for StoreKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let b = bytes.try_into()?;

        Ok(Self {
            key_id: u64::from_be_bytes(b),
        })
    }
}
