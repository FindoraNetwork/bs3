use crate::Result;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Update(Vec<u8>),
    Delete,
}

pub trait Store {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    // fn insert(&self, key: &[u8], value: Vec<u8>) -> Result<Option<Vec<u8>>>;

    fn execute(&self, batch: Vec<(&[u8], &Operation)>) -> Result<()>;
}
