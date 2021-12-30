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

#[test]
fn value_mem_test() -> Result<()> {
    let v = Value::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    assert_eq!(ss.set(1)?, None);
    *ss.get_mut()?.unwrap() += 1;
    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.get()?, Some(Cow::Owned(2)));
    assert_eq!(ss.set(2)?, Some(2));
    *ss.get_mut()?.unwrap() = 20;
    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.get()?, Some(Cow::Owned(20)));

    Ok(())
}

#[test]
fn vec_mem_test() -> Result<()> {
    let v: Vec<i32> = Vec::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();
    assert_eq!(ss.len()?, 0);
    ss.push(1)?;
    ss.push(2)?;
    assert_eq!(ss.len()?, 2);
    assert_eq!(ss.get(1)?, Some(Cow::Borrowed(&2)));

    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.len()?, 2);

    assert_eq!(ss.get(1)?, Some(Cow::Owned(2)));
    ss.push(3)?;
    assert_eq!(ss.len()?, 3);

    assert_eq!(ss.commit()?, 2);

    assert_eq!(ss.pop()?, Some(3));
    assert_eq!(ss.pop()?, Some(2));
    assert_eq!(ss.len()?, 1);
    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.pop()?, Some(1));
    assert_eq!(ss.pop()?, None);

    ss.push(1)?;
    ss.push(2)?;
    assert_eq!(ss.commit()?, 4);
    ss.push(3)?;

    assert_eq!(ss.remove(0)?, Some(1)); //remove invalid, because it has already been submitted

    assert_eq!(ss.get(0)?, Some(Cow::Borrowed(&2)));
    assert_eq!(ss.get(1)?, Some(Cow::Borrowed(&3)));

    *ss.get_mut(0)?.unwrap() += 1;
    *ss.get_mut(1)?.unwrap() += 1;

    assert_eq!(ss.get(1)?, Some(Cow::Borrowed(&4)));

    assert_eq!(ss.commit()?, 5);

    assert_eq!(ss.get(0)?, Some(Cow::Owned(3)));
    assert_eq!(ss.get(1)?, Some(Cow::Owned(4)));
    assert_eq!(ss.get(2)?, None);

    ss.commit()?;

    for _ in 0..100 {
        ss.pop()?;
    }

    assert_eq!(ss.get(0)?, None);
    assert_eq!(ss.get(2)?, None);

    for i in 0..100 {
        if i == 50 {
            ss.commit()?;
        }
        ss.push(i)?;
    }

    assert_eq!(ss.get(0)?, Some(Cow::Owned(0)));
    assert_eq!(ss.get_mut(1)?, Some(&mut 1));
    assert_eq!(ss.len()?, 100);

    Ok(())
}

#[test]
fn doublekeymap_mem_test() -> Result<()> {
    let m: DoubleKeyMap<u32, String, i32> = DoubleKeyMap::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;

    assert_eq!(ss.insert(1, "1".to_string(), 1)?, None);
    assert_eq!(ss.insert(2, "2".to_string(), 2)?, None);
    assert_eq!(ss.insert(3, "3".to_string(), 3)?, None);
    assert_eq!(ss.remove_by_key2("1")?, Some(1));

    assert_eq!(ss.insert(1, "1".to_string(), 1)?, None);

    assert_eq!(ss.commit()?, 1);
    assert_eq!(ss.remove(&1, "2")?, None);

    assert_eq!(ss.commit()?, 2);
    assert_eq!(ss.remove(&1, "1")?, Some(1));

    assert_eq!(ss.commit()?, 3);
    assert_eq!(ss.get(&1, "1")?, None);

    assert_eq!(ss.get_mut(&2, "2")?, Some(&mut 2_i32));
    assert_eq!(ss.get_mut(&2, "3")?, None);

    assert_eq!(ss.get_mut_key1(&2)?, Some(&mut 2_i32));
    assert_eq!(ss.get_mut_key2("2")?, Some(&mut 2_i32));

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

#[test]
fn tx_doublekeymap_mem_test() -> Result<()> {
    let m: DoubleKeyMap<u32, String, i32> = DoubleKeyMap::default();
    let s = MemoryBackend::new();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(m, s)?;
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.insert(1, "1".to_string(), 1)?, None);
    assert_eq!(tx.insert(2, "2".to_string(), 2)?, None);
    assert_eq!(tx.insert(3, "3".to_string(), 3)?, None);

    assert_eq!(tx.remove(&2, "1")?, None);
    assert_eq!(tx.remove_by_key2("1")?, Some(1));

    //&String, &str

    assert_eq!(tx.insert(1, "1".to_string(), 1)?, None);
    assert_eq!(tx.remove(&1, "1")?, Some(1));

    assert_eq!(tx.get(&1, "1")?, None);
    assert_eq!(tx.get_mut(&2, "2")?, Some(&mut 2_i32));

    Ok(())
}

#[test]
fn tx_value_mem_test() -> Result<()> {
    let v: Value<u64> = Value::default();
    let s = MemoryBackend::new();
    let ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();
    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.set(1)?, None);
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&1)));
    assert_eq!(tx.set(2)?, Some(1));
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&2)));

    *tx.get_mut()?.unwrap() += 1;
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&3)));
    *tx.get_mut()?.unwrap() = 10;
    assert_eq!(tx.get()?, Some(Cow::Borrowed(&10)));
    assert_eq!(tx.get_mut()?, Some(&mut 10));

    Ok(())
}

#[test]
fn tx_vec_mem_test() -> Result<()> {
    let v: Vec<i32> = Vec::default();
    let s = MemoryBackend::new();
    let mut ss = SnapshotableStorage::<_, EmptyMerkle<Sha3_512>, _>::new(v, s).unwrap();

    ss.push(1)?;
    ss.push(2)?;
    ss.push(3)?;

    ss.commit()?;

    let mut tx = Transaction::new(&ss);

    assert_eq!(tx.get(0)?, Some(Cow::Owned(1)));
    assert_eq!(tx.get(1)?, Some(Cow::Owned(2)));
    assert_eq!(tx.get(2)?, Some(Cow::Owned(3)));

    tx.push(4)?;
    tx.push(5)?;

    assert_eq!(tx.get(3)?, Some(Cow::Borrowed(&4)));
    assert_eq!(tx.get(4)?, Some(Cow::Borrowed(&5)));

    assert_eq!(tx.remove(0)?, Some(1));
    assert_eq!(tx.len()?, 4);

    tx.insert(0, 10)?;
    tx.insert(1, 20)?;

    assert_eq!(tx.len()?, 6);

    assert_eq!(tx.get(1)?, Some(Cow::Borrowed(&20)));
    assert_eq!(tx.get(2)?, Some(Cow::Borrowed(&2)));
    Ok(())
}

fn main() -> Result<()> {
    map_mem_test()?;
    value_mem_test()?;
    vec_mem_test()?;
    doublekeymap_mem_test()?;

    tx_map_mem_test()?;
    tx_value_mem_test()?;
    tx_vec_mem_test()?;
    tx_doublekeymap_mem_test()?;
    Ok(())
}
