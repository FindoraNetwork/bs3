use alloc::boxed::Box;
use core::marker::PhantomData;
use alloc::vec::Vec;
use digest::generic_array::GenericArray;
use digest::Output;
use sha3::Digest;

use crate::{OperationBytes, Result, Store, Error, Operation};
use crate::snapshot::{FromStoreBytes, StoreValue, ToStoreBytes};

use super::Merkle;
use super::min;

pub struct AppendOnlyMerkle<D: Digest> {
    marker: PhantomData<D>,
}

impl<D: Digest> Default for AppendOnlyMerkle<D> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<D: Digest> Merkle for AppendOnlyMerkle<D> {
    type Digest = D;

    fn insert<S: Store>(&mut self,
                        prev_key: Vec<u8>,
                        cur_key: Vec<u8>,
                        store: &mut S,
                        batch: &[(Vec<u8>, OperationBytes)])
        -> Result<()> {

        let mut hashs = Vec::new();
        let mut hasher = D::new();

        for (key,value) in batch.iter() {
            hasher.update(key);
            match value {
                OperationBytes::Update(v) => {hasher.update(v);}
                OperationBytes::Delete => {hasher.update(Vec::new());}
            };
            let hash = hasher.finalize_reset()[..].to_vec();
            hashs.push(hash);
        }

        if let Some(output) = self.root(prev_key, store)? {
            let prev_root = output[..].to_vec();
            hashs.push(prev_root);
        } else {
            log::debug!("get prev root is none.");
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
                hasher.update(left_hash);
                hasher.update(right_hash);
                let hash = hasher.finalize_reset()[..].to_vec();
                hashs.push(hash);

                left += 2;
            }
            offset += num_of_layers;
            num_of_layers = (num_of_layers + 1)/2;
        }
        let operation = Operation::Update(hashs);
        let value = StoreValue{ operation:operation.to_bytes()? };
        store.insert(cur_key, value.to_bytes()?)?;

        Ok(())
    }

    fn root<S: Store>(&self, key: Vec<u8>, store: &S) -> Result<Option<Output<D>>> {
        if let Some(bytes) = store.get_ge(key.as_slice())? {
            let value = StoreValue::from_bytes(&bytes)?;
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

