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
    use crate::Result;
    use crate::{Cow, MapStore, SnapshotableStorage, Store, ValueStore};
    use alloc::string::{String, ToString};
    use core::ops::Deref;

    #[test]
    fn map_mem_test() -> Result<()> {
        let m = Map::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(m, s)?;

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

    #[test]
    fn value_mem_test() -> Result<()> {
        let v = Value::default();
        let s = MemoryBackend::new();
        let mut ss = SnapshotableStorage::new(v, s).unwrap();

        assert_eq!(ss.set(1)?, None);
        assert_eq!(ss.commit()?, 1);
        assert_eq!(ss.get()?, Some(Cow::Owned(1)));
        assert_eq!(ss.set(2)?, Some(1));
        assert_eq!(ss.commit()?, 2);
        assert_eq!(ss.get()?, Some(Cow::Owned(2)));

        Ok(())
    }

    #[test]
    fn vec_mem_test() -> Result<()> {
        let v = Vec::default();
        let s = MemoryBackend::new();
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
}
