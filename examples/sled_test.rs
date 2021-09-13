use bs3::backend::{sled_db_open, SledBackend};
use bs3::model::{Map, Value, Vec};
use bs3::SnapshotableStorage;
use bs3::{Cow, MapStore, Result, ValueStore, VecStore};
use core::ops::Deref;
use std::string::{String, ToString};

fn sled_vec_test() -> Result<()> {
    let v = Vec::default();
    let db = sled_db_open("/tmp/vec_sled_test/05", false).unwrap();
    let s = SledBackend::open_tree(&db, "vec_sled_test").unwrap();
    let mut ss = SnapshotableStorage::new(v, s).unwrap();

    assert_eq!(ss.insert(1)?, None);
    assert_eq!(ss.insert(2)?, None);
    assert_eq!(ss.insert(3)?, None);
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.remove(0)?, None); //remove invalid, because it has already been submitted
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get(0)?, Some(Cow::Owned(1)));
    assert_eq!(ss.get(1)?, Some(Cow::Owned(2)));
    assert_eq!(ss.get(2)?, Some(Cow::Owned(3)));
    Ok(())
}

fn sled_map_test() -> Result<()> {
    let m = Map::default();
    let db = sled_db_open("/tmp/map_sled_test/05", false).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let mut ss = SnapshotableStorage::new(m, s).unwrap();

    assert_eq!(ss.insert(1, 1)?, None);
    assert_eq!(ss.insert(2, 2)?, None);
    assert_eq!(ss.insert(3, 3)?, None);
    assert_eq!(ss.remove(1)?, None); //remove valid, thought not submitted before deletion
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1)?, None);
    assert_eq!(ss.get_mut(2)?, Some(&mut 2_i32));

    Ok(())
}

fn sled_value_test() -> Result<()> {
    let v = Value::default();
    let db = sled_db_open("/tmp/value_sled_test/05", false).unwrap();
    let s = SledBackend::open_tree(&db, "value_sled_test").unwrap();
    let mut ss = SnapshotableStorage::new(v, s).unwrap();

    assert_eq!(ss.set(1)?, None);
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.get()?, Some(Cow::Owned(1)));
    assert_eq!(ss.set(2)?, Some(1));
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get()?, Some(Cow::Owned(2)));

    Ok(())
}

fn main() {
    sled_vec_test();
    sled_map_test();
    sled_value_test();
}
