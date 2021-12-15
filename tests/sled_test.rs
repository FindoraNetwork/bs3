use bs3::backend::{sled_db_open, SledBackend};
use bs3::merkle::empty::EmptyMerkle;
use bs3::model::{DoubleKeyMap, Map, Value, Vec};
use bs3::prelude::Tree;
use bs3::{Cow, DoubleKeyMapStore, MapStore, Result, ValueStore, VecStore};
use bs3::{SnapshotableStorage, Transaction};
use sha3::Sha3_512;

fn sled_vec_test() -> Result<()> {
    let v = Vec::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "vec_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    assert_eq!(ss.insert(1)?, None);
    assert_eq!(ss.insert(2)?, None);
    assert_eq!(ss.insert(3)?, None);
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.remove(0)?, Some(1));
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get(0)?, None);
    assert_eq!(ss.get(1)?, Some(Cow::Owned(2)));
    assert_eq!(ss.get(2)?, Some(Cow::Owned(3)));

    ss.rollback(2)?;
    let key = serde_json::value::Number::from(2);
    let key = serde_json::to_vec(&key).unwrap();
    assert_eq!(ss.tree_get(&key, 2)?, 51_u8.to_be_bytes().to_vec());

    Ok(())
}

fn sled_map_test() -> Result<()> {
    let m = Map::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();

    assert_eq!(ss.insert(1, 1)?, None);
    assert_eq!(ss.insert(2, 2)?, None);
    assert_eq!(ss.insert(3, 3)?, None);
    assert_eq!(ss.remove(&1)?, Some(1)); //remove valid, thought not submitted before deletion
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1)?, None);
    assert_eq!(ss.get_mut(&2)?, Some(&mut 2_i32));

    ss.rollback(1)?;

    let key = serde_json::value::Number::from(2);
    let key = serde_json::to_vec(&key).unwrap();

    assert_eq!(ss.tree_get(&key, 1)?, 50_u8.to_be_bytes().to_vec());

    Ok(())
}

fn sled_doublekeymap_test() -> Result<()> {
    let m = DoubleKeyMap::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();

    assert_eq!(ss.insert(1, 1, 1)?, None);
    assert_eq!(ss.insert(2, 2, 2)?, None);
    assert_eq!(ss.insert(3, 3, 3)?, None);
    assert_eq!(ss.remove(&1, &1)?, Some(1)); //remove valid, thought not submitted before deletion
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1, &1)?, None);
    assert_eq!(ss.get_mut(&2, &2)?, Some(&mut 2_i32));

    ss.rollback(1)?;

    let key = serde_json::to_vec(&(2, 2)).unwrap();

    assert_eq!(ss.tree_get(&key, 1)?, 50_u8.to_be_bytes().to_vec());
    Ok(())
}

fn sled_value_test() -> Result<()> {
    let v = Value::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "value_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    assert_eq!(ss.set(1)?, None);
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.get()?, Some(Cow::Owned(1)));
    assert_eq!(ss.set(2)?, Some(1));
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get()?, Some(Cow::Owned(2)));

    ss.rollback(2)?;

    assert_eq!(ss.tree_get(&vec![], 2)?, 50_u8.to_be_bytes().to_vec());

    Ok(())
}

fn tx_sled_value_test() -> Result<()> {
    let v = Value::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "value_sled_test").unwrap();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.set(1)?, None);
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&1)));
    assert_eq!(tx.set(2)?, Some(1));
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&2)));

    Ok(())
}

fn tx_sled_map_test() -> Result<()> {
    let m = Map::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1, 1)?, None);
    assert_eq!(tx.insert(2, 2)?, None);
    assert_eq!(tx.insert(3, 3)?, None);
    assert_eq!(tx.remove(&1)?, Some(1)); //remove valid, thought not submitted before deletion
    assert_eq!(tx.get(&1)?, None);
    assert_eq!(tx.get_mut(&2)?, Some(&mut 2_i32));

    Ok(())
}

fn tx_sled_doublekeymap_test() -> Result<()> {
    let m = DoubleKeyMap::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1, 1, 1)?, None);
    assert_eq!(tx.insert(2, 2, 2)?, None);
    assert_eq!(tx.insert(3, 3, 3)?, None);
    assert_eq!(tx.remove(&1, &1)?, Some(1));
    assert_eq!(tx.get(&1, &1)?, None);
    assert_eq!(tx.get_mut(&2, &2)?, Some(&mut 2_i32));

    Ok(())
}

fn tx_sled_vec_test() -> Result<()> {
    let v = Vec::default();
    let db = sled_db_open(None).unwrap();
    let s = SledBackend::open_tree(&db, "vec_sled_test").unwrap();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1)?, None);
    assert_eq!(tx.insert(2)?, None);
    assert_eq!(tx.insert(3)?, None);
    assert_eq!(tx.remove(0)?, Some(1));
    assert_eq!(tx.get(0)?, None);
    assert_eq!(tx.get(1)?, Some(Cow::Borrowed(&2)));
    assert_eq!(tx.get(2)?, Some(Cow::Borrowed(&3)));

    Ok(())
}

fn sled_vec_test_reload_and_callback(is_rollback: bool) -> Result<()> {
    let v = Vec::default();
    let db = sled_db_open(Some("/tmp/bs3_test/vec_test")).unwrap();
    let s = SledBackend::open_tree(&db, "vec_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    if is_rollback {
        ss.rollback(1)?;
        assert_eq!(ss.get(0)?, Some(Cow::Owned(1)));
        assert_eq!(ss.get(1)?, Some(Cow::Owned(2)));
        assert_eq!(ss.get(2)?, Some(Cow::Owned(3)));
        assert_eq!(ss.commit()?, 2);
    } else {
        assert_eq!(ss.insert(1)?, None);
        assert_eq!(ss.insert(2)?, None);
        assert_eq!(ss.insert(3)?, None);
        assert_eq!(ss.commit()?, 1);
        assert_eq!(ss.remove(0)?, None); //remove invalid, because it has already been submitted
        assert_eq!(ss.commit()?, 2);
        assert_eq!(ss.get(0)?, Some(Cow::Owned(1)));
        assert_eq!(ss.get(1)?, Some(Cow::Owned(2)));
        assert_eq!(ss.get(2)?, Some(Cow::Owned(3)));
    }

    Ok(())
}

fn sled_map_test_reload_and_callback(is_rollback: bool) -> Result<()> {
    let m = Map::default();
    let db = sled_db_open(Some("/tmp/bs3_test/map_test")).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();

    if is_rollback {
        ss.rollback(1)?;
        assert_eq!(ss.get(&1)?, None);
        assert_eq!(ss.get_mut(&2)?, Some(&mut 2_i32));
        assert_eq!(ss.commit()?, 2);
    } else {
        assert_eq!(ss.insert(1, 1)?, None);
        assert_eq!(ss.insert(2, 2)?, None);
        assert_eq!(ss.insert(3, 3)?, None);
        assert_eq!(ss.remove(&1)?, Some(1)); //remove valid, thought not submitted before deletion
        assert_eq!(ss.commit()?, 1);
        assert_eq!(ss.commit()?, 2);
        assert_eq!(ss.commit()?, 3);
        assert_eq!(ss.get(&1)?, None);
        assert_eq!(ss.get_mut(&2)?, Some(&mut 2_i32));
    }

    Ok(())
}

fn sled_value_test_reload_and_callback(is_rollback: bool) -> Result<()> {
    let v = Value::default();
    let db = sled_db_open(Some("/tmp/bs3_test/value_test")).unwrap();
    let s = SledBackend::open_tree(&db, "value_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    if is_rollback {
        ss.rollback(1)?;
        assert_eq!(ss.get()?, Some(Cow::Owned(1)));
        assert_eq!(ss.commit()?, 2);
    } else {
        assert_eq!(ss.set(1)?, None);
        assert_eq!(ss.commit()?, 1);
        assert_eq!(ss.get()?, Some(Cow::Owned(1)));
        assert_eq!(ss.set(2)?, Some(1));
        assert_eq!(ss.commit()?, 2);
        assert_eq!(ss.get()?, Some(Cow::Owned(2)));
    }

    Ok(())
}

fn sled_doublekeymap_test_reload_and_callback(is_rollback: bool) -> Result<()> {
    let m = DoubleKeyMap::default();
    let db = sled_db_open(Some("/tmp/bs3_test/doublekeymap_test")).unwrap();
    let s = SledBackend::open_tree(&db, "map_sled_test").unwrap();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s).unwrap();

    if is_rollback {
        ss.rollback(1)?;
        assert_eq!(ss.get(&1, &1)?, None);
        assert_eq!(ss.get_mut(&2, &2)?, Some(&mut 2_i32));
        assert_eq!(ss.commit()?, 2);
    } else {
        assert_eq!(ss.insert(1, 1, 1)?, None);
        assert_eq!(ss.insert(2, 2, 2)?, None);
        assert_eq!(ss.insert(3, 3, 3)?, None);
        assert_eq!(ss.remove(&1, &1)?, Some(1)); //remove valid, thought not submitted before deletion
        assert_eq!(ss.commit()?, 1);
        assert_eq!(ss.commit()?, 2);
        assert_eq!(ss.commit()?, 3);
        assert_eq!(ss.get(&1, &1)?, None);
        assert_eq!(ss.get_mut(&2, &2)?, Some(&mut 2_i32));
    }

    Ok(())
}

fn main() {
    let _ = sled_vec_test_reload_and_callback(false);
    let _ = sled_vec_test_reload_and_callback(true);

    let _ = sled_map_test_reload_and_callback(false);
    let _ = sled_map_test_reload_and_callback(true);

    let _ = sled_value_test_reload_and_callback(false);
    let _ = sled_value_test_reload_and_callback(true);

    let _ = sled_doublekeymap_test_reload_and_callback(false);
    let _ = sled_doublekeymap_test_reload_and_callback(true);

    let _ = sled_vec_test();
    let _ = sled_map_test();
    let _ = sled_value_test();
    let _ = sled_doublekeymap_test();

    let _ = tx_sled_value_test();
    let _ = tx_sled_map_test();
    let _ = tx_sled_vec_test();
    let _ = tx_sled_doublekeymap_test();
}
