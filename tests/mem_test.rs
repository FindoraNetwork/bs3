use bs3::backend::MemoryBackend;
use bs3::merkle::empty::EmptyMerkle;
use bs3::model::{DoubleKeyMap, Map, Value, Vec};
use bs3::{Cow, DoubleKeyMapStore, MapStore, Result, ValueStore, VecStore};
use bs3::{SnapshotableStorage, Transaction};
use sha3::Sha3_512;

fn map_mem_test() -> Result<()> {
    let m = Map::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;

    assert_eq!(ss.insert(1, 1)?, None);
    assert_eq!(ss.insert(2, 2)?, None);
    assert_eq!(ss.insert(3, 3)?, None);
    assert_eq!(ss.remove(&1)?, Some(1)); //remove valid, thought not submitted before deletion
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1)?, None);
    assert_eq!(ss.get_mut(&2)?, Some(&mut 2_i32));

    Ok(())
}

fn value_mem_test() -> Result<()> {
    let v = Value::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    assert_eq!(ss.set(1)?, None);
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.get()?, Some(Cow::Owned(1)));
    assert_eq!(ss.set(2)?, Some(1));
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get()?, Some(Cow::Owned(2)));

    Ok(())
}

fn vec_mem_test() -> Result<()> {
    let v = Vec::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

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

fn doublekeymap_mem_test() -> Result<()> {
    let m = DoubleKeyMap::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;

    assert_eq!(ss.insert(1, 1, 1)?, None);
    assert_eq!(ss.insert(2, 2, 2)?, None);
    assert_eq!(ss.insert(3, 3, 3)?, None);
    assert_eq!(ss.remove(&1, &1)?, Some(1)); //remove valid, thought not submitted before deletion
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1, &1)?, None);
    assert_eq!(ss.get_mut(&2, &2)?, Some(&mut 2_i32));

    Ok(())
}

fn tx_map_mem_test() -> Result<()> {
    let m = Map::default();
    let s = MemoryBackend::new();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1, 1)?, None);
    assert_eq!(tx.insert(2, 2)?, None);
    assert_eq!(tx.insert(3, 3)?, None);
    assert_eq!(tx.remove(&1)?, Some(1));
    assert_eq!(tx.get(&1)?, None);
    assert_eq!(tx.get_mut(&2)?, Some(&mut 2_i32));

    Ok(())
}

fn tx_doublekeymap_mem_test() -> Result<()> {
    let m = DoubleKeyMap::default();
    let s = MemoryBackend::new();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1, 1, 1)?, None);
    assert_eq!(tx.insert(2, 2, 2)?, None);
    assert_eq!(tx.insert(3, 3, 3)?, None);
    assert_eq!(tx.remove(&1, &1)?, Some(1));
    assert_eq!(tx.get(&1, &1)?, None);
    assert_eq!(tx.get_mut(&2, &2)?, Some(&mut 2_i32));

    Ok(())
}

fn tx_value_mem_test() -> Result<()> {
    let v = Value::default();
    let s = MemoryBackend::new();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.set(1)?, None);
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&1)));
    assert_eq!(tx.set(2)?, Some(1));
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&2)));

    Ok(())
}

fn tx_vec_mem_test() -> Result<()> {
    let v = Vec::default();
    let s = MemoryBackend::new();
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

fn main() {
    let _ = map_mem_test();
    let _ = value_mem_test();
    let _ = vec_mem_test();
    let _ = doublekeymap_mem_test();

    let _ = tx_map_mem_test();
    let _ = tx_value_mem_test();
    let _ = tx_vec_mem_test();
    let _ = tx_doublekeymap_mem_test();
}
