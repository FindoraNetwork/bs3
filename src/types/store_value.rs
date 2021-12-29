use alloc::vec::Vec;

use crate::{
    prelude::{FromBytes, ToBytes},
    Error, Result,
};

#[derive(Debug, Clone)]
pub enum StoreValue {
    Update(Vec<u8>),
    Delete,
    Ref { branch_id: u64, version_id: u64 },
}

const STORE_VALUE_TYPE_INDEX_UPDATE: u8 = 1;
const STORE_VALUE_TYPE_INDEX_DELETE: u8 = 2;
const STORE_VALUE_TYPE_INDEX_REF: u8 = 3;

impl ToBytes for StoreValue {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut res = Vec::new();
        match self {
            StoreValue::Update(d) => {
                res.push(STORE_VALUE_TYPE_INDEX_UPDATE);
                res.extend_from_slice(&d);
            }
            StoreValue::Delete => res.push(STORE_VALUE_TYPE_INDEX_DELETE),
            StoreValue::Ref {
                branch_id,
                version_id,
            } => {
                res.push(STORE_VALUE_TYPE_INDEX_REF);
                res.extend_from_slice(&branch_id.to_be_bytes());
                res.extend_from_slice(&version_id.to_be_bytes());
            }
        }
        Ok(res)
    }
}

impl FromBytes for StoreValue {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        if bytes.len() < 1 {
            return Err(Error::BytesLengthError);
        }
        let type_code = &bytes[0];
        match type_code {
            0 => {
                let data = &bytes[1..];
                return Ok(StoreValue::Update(data.to_vec()));
            }
            1 => return Ok(StoreValue::Delete),
            2 => {
                let branch_id_bytes = &bytes[1..];
                let branch_id_array = branch_id_bytes.try_into()?;
                let branch_id = u64::from_be_bytes(branch_id_array);

                let version_id_bytes = &bytes[1..];
                let version_id_array = version_id_bytes.try_into()?;
                let version_id = u64::from_be_bytes(version_id_array);

                return Ok(StoreValue::Ref {
                    branch_id,
                    version_id,
                });
            }
            _ => return Err(Error::TypeCodeError),
        }
    }
}
