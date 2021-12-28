use alloc::vec::Vec;

use crate::Result;

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bytes.to_vec())
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.clone())
    }
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
            Self: Sized {
        let b: [u8; 8] = bytes.try_into()?;

        Ok(u64::from_be_bytes(b))
    }
}

impl ToBytes for u64 {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let b = self.to_be_bytes();
        Ok(b.to_vec())
    }
}
