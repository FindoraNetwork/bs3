use crate::{
    merkle::Merkle,
    model::{ValueType, Vec, INDEX_VEC_LEN},
    snapshot::{FromStoreBytes, SnapshotableStorage, StoreValue},
    utils::cbor_encode,
    Cow, Error, Operation, Result, Store,
};

use super::utils::get_greatest;

pub trait VecStore<T: ValueType> {
    fn get(&self, index: u64) -> Result<Option<Cow<'_, T>>>;

    fn get_mut(&mut self, index: u64) -> Result<Option<&mut T>>;

    fn push(&mut self, value: T) -> Result<()>;

    fn pop(&mut self) -> Result<Option<T>>;

    fn remove(&mut self, index: u64) -> Result<Option<T>> {
        let len = self.len()?;
        if index >= len {
            return Err(Error::OutOffIndex);
        }

        // len will > 1 here.
        let mut poped = alloc::vec::Vec::with_capacity(64);
        while self.len()? > index {
            poped.push(self.pop()?)
        }

        let res = poped.pop().unwrap();

        while let Some(Some(t)) = poped.pop() {
            self.push(t)?;
        }
        Ok(res)
    }

    fn insert(&mut self, index: u64, value: T) -> Result<()> {
        if index > self.len()? {
            return Err(Error::OutOffIndex);
        }

        let mut poped = alloc::vec::Vec::with_capacity(64);

        while self.len()? > index {
            poped.push(self.pop()?)
        }

        self.push(value)?;
        while let Some(Some(t)) = poped.pop() {
            self.push(t)?;
        }

        Ok(())
    }

    fn len(&self) -> Result<u64>;

    fn is_empty(&self) -> Result<bool> {
        self.len().map(|x| x == 0)
    }
}

impl<S, M, T> VecStore<T> for SnapshotableStorage<S, M, Vec<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn len(&self) -> Result<u64> {
        if let Some(l) = self.value.current_len() {
            return Ok(l);
        }
        self.get_len()
    }

    fn get(&self, index: u64) -> Result<Option<Cow<'_, T>>> {
        let len = self.len()?;
        if index >= len || len == 0 {
            return Ok(None);
        }

        if let Some(operation) = self.value.get(&index) {
            match operation {
                Operation::Update(v) => Ok(Some(Cow::Borrowed(v))),
                Operation::Delete => Ok(None),
            }
        } else {
            match self.get_inner_value(index)? {
                None => Ok(None),
                Some(v) => Ok(Some(Cow::Owned(v))),
            }
        }
    }

    fn get_mut(&mut self, index: u64) -> Result<Option<&mut T>> {
        let len = self.len()?;
        if index >= len || len == 0 {
            return Ok(None);
        }

        if self.value.contains_key(&index) {
            match self.value.get_mut(&index).unwrap() {
                Operation::Delete => Ok(None),
                Operation::Update(t) => Ok(Some(t)),
            }
        } else {
            match self.get_inner_value(index)? {
                Some(t) => {
                    self.value.insert_operation(index, Operation::Update(t));
                    match self.value.get_mut(&index).unwrap() {
                        Operation::Update(t) => Ok(Some(t)),
                        _ => unreachable!(),
                    }
                }
                None => Ok(None),
            }
        }
    }
    //HEAD
    fn push(&mut self, value: T) -> Result<()> {
        let len = self.len()?;
        self.value.insert_operation(len, Operation::Update(value));
        self.value.set_length(len + 1);
        self.value.add_offset();
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<T>> {
        let len = self.len()?;

        if len == 0 {
            return Ok(None);
        }

        self.value.set_length(len - 1);
        self.value.sub_offset();

        if let Some(Operation::Update(t)) = self.value.remove_operation(len - 1) {
            return Ok(Some(t));
        };

        let res = self.get_inner_value(len - 1)?;
        debug_assert!(res.is_some());
        Ok(res)
    }
}

impl<S, M, T> SnapshotableStorage<S, M, Vec<T>>
where
    S: Store,
    M: Merkle,
    T: ValueType,
{
    pub fn get_inner_value(&self, index: u64) -> Result<Option<T>>
    where
        T: ValueType,
        S: Store,
        M: Merkle,
    {
        let key_bytes = cbor_encode(&index)?;
        let store_key = self.storage_tuple_key(&key_bytes);
        get_greatest(&self.store, &store_key.0, &store_key.1)
    }

    pub fn get_len(&self) -> Result<u64>
    where
        T: ValueType,
        S: Store,
        M: Merkle,
    {
        let key_bytes = cbor_encode(&INDEX_VEC_LEN)?;
        let store_key = self.storage_tuple_key(&key_bytes);

        match self.store.get_ge2((&store_key.0, &store_key.1))? {
            Some(bytes) => {
                let value = StoreValue::from_bytes(&bytes)?;
                let operation = Operation::from_bytes(&value.operation)?;
                match operation {
                    Operation::Update(t) => Ok(t),
                    _ => panic!("maybee a impl bug, unreachable, `len` always set"),
                }
            }
            None => Ok(0),
        }
    }
}
