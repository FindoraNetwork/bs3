use super::utils::vec_utils;
use crate::{
    merkle::Merkle,
    model::{ValueType, Vec, INDEX_VEC_LEN},
    Cow, Error, Operation, Result, SnapshotableStorage, Store,
};
use serde::{Deserialize, Serialize};

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
}

impl<S, M, T> VecStore<T> for SnapshotableStorage<S, M, Vec<T>>
where
    T: ValueType,
    S: Store,
    M: Merkle,
{
    fn len(&self) -> Result<u64> {
        if let Some(l) = self.value.len() {
            return Ok(l);
        }
        Ok(vec_utils::get_len(self)?)
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
            match vec_utils::get_inner_value(self, index)? {
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
            match vec_utils::get_inner_value(self, index)? {
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
        self.value.set_len(len + 1);
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<T>> {
        let len = self.len()?;

        if len == 0 {
            return Ok(None);
        }

        self.value.set_len(len - 1);

        match self.value.remove_operation(len - 1) {
            Some(Operation::Update(t)) => {
                return Ok(Some(t));
            }
            _ => (),
        };

        let res = vec_utils::get_inner_operation(self, len - 1)?;
        match res {
            Some(Operation::Update(t)) => Ok(Some(t)),
            _ => unreachable!(),
        }
    }
}
