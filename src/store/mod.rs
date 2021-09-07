mod value;
pub use value::ValueStore;

mod map;
pub use map::MapStore;

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
    fn map_test() {
        let m = Map::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(m, s).unwrap();
        let r = ss.insert(1, 1);
        std::println!("{:?}", r);
        let r = ss.insert(2, 2);
        std::println!("{:?}", r);
        let r = ss.insert(3, 3);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.remove(1);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.get(&1);
        std::println!("{:?}", r);
        let r = ss.get_mut(1);
        std::println!("{:?}", r);
    }

    #[test]
    fn value_test() {
        let v = Value::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(v, s).unwrap();
        let r = ss.set(1);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.get();
        std::println!("{:?}", r);
        let r = ss.set(2);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.get();
        std::println!("{:?}", r);
    }

    #[test]
    fn vec_test() {
        let v = Vec::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(v, s).unwrap();
        let r = ss.insert(1);
        std::println!("{:?}", r);
        let r = ss.insert(2);
        std::println!("{:?}", r);
        let r = ss.insert(3);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.remove(0);
        std::println!("{:?}", r);
        let r = ss.commit();
        std::println!("{:?}", r);
        let r = ss.get(0).unwrap().unwrap();
        std::println!("{:?}", r.deref().clone());
        let r = ss.get(1).unwrap().unwrap();
        std::println!("{:?}", r.deref().clone());
        let r = ss.get(2).unwrap().unwrap();
        std::println!("{:?}", r.deref().clone());
    }
}
