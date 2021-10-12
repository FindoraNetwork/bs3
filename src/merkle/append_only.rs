use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::marker::PhantomData;
use alloc::vec::Vec;
use digest::generic_array::GenericArray;
use digest::Output;
use sha3::Digest;
use crate::snapshot::utils::merkle_key;

use crate::{OperationBytes, Result, Store, Error, Operation};
use crate::merkle::value::MerkleValue;
use crate::snapshot::{FromStoreBytes, ToStoreBytes};

use super::Merkle;
use super::min;

pub struct AppendOnlyMerkle<D: Digest> {
    hasher: D,
    namespace: String,
    height: i64,
}

impl<D: Digest> Default for AppendOnlyMerkle<D> {
    fn default() -> Self {
        Self {
            hasher: D::new(),
            namespace: String::default(),
            height: 0,
        }
    }
}

impl<D: Digest> Merkle for AppendOnlyMerkle<D> {
    type Digest = D;

    fn rollback(&mut self, target_height: i64) -> Result<()> {
        if target_height > self.height {
            log::error!(
                "Target height {} must less than current height {}",
                target_height,
                self.height
            );
            Err(Error::HeightError)
        } else {
            self.height = target_height;
            Ok(())
        }
    }

    fn new(namespace: &str, height: i64) -> Self {
        AppendOnlyMerkle{
            hasher: D::new(),
            namespace: namespace.to_string(),
            height
        }
    }

    fn insert<S: Store>(&mut self, store: &mut S, batch: &[(Vec<u8>, OperationBytes)])
        -> Result<()> {

        let mut hashs = Vec::new();

        if let Some(output) = self.root(store)? {
            let prev_root = output[..].to_vec();
            hashs.push(prev_root);
        } else {
            log::debug!("get prev root is none, height:{}",self.height);
        }

        for (key,value) in batch.iter() {
            self.hasher.update(key);
            match value {
                OperationBytes::Update(v) => {self.hasher.update(v);}
                OperationBytes::Delete => {self.hasher.update(Vec::new());}
            };
            let hash = self.hasher.finalize_reset()[..].to_vec();
            hashs.push(hash);
        }

        if hashs.len()%2 != 0 {
            hashs.push(hashs.last().unwrap().clone());
        }

        let mut num_of_layers = hashs.len();
        let mut offset = 0_usize;
        while num_of_layers > 1 {

            let mut left = 0;
            while left < num_of_layers {
                let right = min(left + 1,num_of_layers - 1);
                let left_hash = hashs.get(offset + left).unwrap();
                let right_hash = hashs.get(offset + right).unwrap();
                self.hasher.update(left_hash);
                self.hasher.update(right_hash);
                let hash = self.hasher.finalize_reset()[..].to_vec();
                hashs.push(hash);

                left += 2;
            }
            offset += num_of_layers;
            num_of_layers = (num_of_layers + 1)/2;
        }
        let operation = Operation::Update(hashs);
        let value = MerkleValue{ operation:operation.to_bytes()? };

        self.height += 1;
        let cur_key = merkle_key(&*self.namespace, self.height);
        store.insert(cur_key, value.to_bytes()?)?;

        Ok(())
    }

    fn root<S: Store>(&self, store: &S) -> Result<Option<Output<D>>> {
        let key = merkle_key(&*self.namespace, self.height);
        if let Some(bytes) = store.get_ge(key.as_slice())? {
            let value = MerkleValue::from_bytes(&bytes)?;
            if let Operation::Update(hashs) =
                Operation::<Vec<Vec<u8>>>::from_bytes(&value.operation)? {
                if let Some(root) = hashs.last() {
                    let array = GenericArray::<u8,D::OutputSize>::from_slice(root.as_slice());
                    Ok(Some(array.clone()))
                } else {
                    Err(Error::StoreError(Box::new("this merkle size is 0")))
                }

            } else {
                Err(Error::StoreError(Box::new("this operation is delete")))
            }

        } else {
            Ok(None)
        }
    }
}

