use crate::Result;

pub trait Merkle {
    fn insert(key: &[u8], value: &[u8]) -> Result<()>;
}
