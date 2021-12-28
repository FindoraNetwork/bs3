use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct BranchName(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct VersionName(pub Vec<u8>);
