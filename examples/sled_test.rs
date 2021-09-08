use bs3::backend::{sled_db_open, SledBackend};
use bs3::model::{Map, Value, Vec};
use bs3::SnapshotableStorage;
use bs3::{MapStore, ValueStore, VecStore};
use core::ops::Deref;
use std::string::{String, ToString};

fn sled_vec_test() {
    let v = Vec::default();
    let db = sled_db_open("/tmp/vec_sled_test/01", false).unwrap();
    let s = SledBackend::open_tree(&db, "vec_sled_test").unwrap();
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

fn sled_map_test() {
    let m = Map::default();
    let db = sled_db_open("/tmp/map_sled_test/01", false).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
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

fn sled_value_test() {
    let v = Value::default();
    let db = sled_db_open("/tmp/value_sled_test/01", false).unwrap();
    let s = SledBackend::open_tree(&db, "value_sled_test").unwrap();
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

fn main() {
    sled_vec_test();
    sled_map_test();
    sled_value_test();
}
