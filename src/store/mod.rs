mod value;
pub use value::ValueStore;

mod map;
pub use map::MapStore;

mod tx;
mod utils;
mod vec;

pub use vec::VecStore;

#[cfg(test)]
mod tests {

    use crate::backend::MemoryBackend;
    use crate::model::{Map, Value, Vec};
    use crate::store::VecStore;
    use crate::{Cow, MapStore, SnapshotableStorage, Store, ValueStore};
    use alloc::string::{String, ToString};
    use core::ops::Deref;

    #[test]
    fn map_mem_test() {
        let m = Map::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(m, s).unwrap();
        let r = ss.insert(1, 1);
        let r = ss.insert(2, 2);
        let r = ss.insert(3, 3);
        let r = ss.commit();
        let r = ss.remove(1);
        let r = ss.commit();
        let r = ss.get(&1);
        let r = ss.get_mut(1);
    }

    #[test]
    fn value_mem_test() {
        let v = Value::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(v, s).unwrap();
        let r = ss.set(1);
        let r = ss.commit();
        let r = ss.get();
        let r = ss.set(2);
        let r = ss.commit();
        let r = ss.get();
    }

    #[test]
    fn vec_mem_test() {
        let v = Vec::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(v, s).unwrap();
        let r = ss.insert(1);
        let r = ss.insert(2);
        let r = ss.insert(3);
        let r = ss.commit();
        let r = ss.remove(0);
        let r = ss.commit();
        let r = ss.get(0);
        let r = ss.get(1);
        let r = ss.get(2);
    }
}
