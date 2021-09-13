mod map;
mod value;
mod vec;

use super::utils;

#[cfg(test)]
mod tests {

    use crate::backend::MemoryBackend;
    use crate::model::{Map, Value, Vec};
    use crate::store::VecStore;
    use crate::{Cow, Error, MapStore, SnapshotableStorage, Store, Transaction, ValueStore};
    use alloc::string::{String, ToString};
    use core::ops::Deref;

    #[test]
    fn tx_map_mem_test() {
        let m = Map::default();
        let s = MemoryBackend::new();
        let ss = SnapshotableStorage::new(m, s).unwrap();
        let mut tx = Transaction::new(&ss);

        let r = tx.insert(1, 1); //Err(CborDeIoError("Semantic(None, \"missing field `operation`\")"))
        let r = tx.insert(2, 2);
        let r = tx.insert(3, 3);
        let r = tx.remove(1);
        let r = tx.get(&1);
        let r = tx.get_mut(1);
        let r = tx.get(&2);
        let r = tx.get(&3);
    }

    #[test]
    fn tx_value_mem_test() {
        let v = Value::default();
        let s = MemoryBackend::new();
        let ss = SnapshotableStorage::new(v, s).unwrap();
        let mut tx = Transaction::new(&ss);

        let r = tx.set(1);
        let r = tx.get();
        let r = tx.set(2);
        let r = tx.get();
    }

    #[test]
    fn tx_vec_mem_test() {
        let v = Vec::default();
        let s = MemoryBackend::new();
        let ss = SnapshotableStorage::new(v, s).unwrap();
        let mut tx = Transaction::new(&ss);

        let r = tx.insert(1);
        let r = tx.insert(2);
        let r = tx.insert(3);
        let r = tx.remove(0);
        let r = tx.get(0);
        let r = tx.get(1);
        let r = tx.get(2);
    }
}
