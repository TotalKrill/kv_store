use crate::{KeyValueStore, MutKeyValueStore};
use parking_lot::{Mutex, RwLock};
use std::{collections::BTreeMap, error::Error, sync::Arc};

pub struct RwMutexMap<K, V>(pub RwLock<BTreeMap<K, Arc<Mutex<V>>>>)
where
    V: Clone,
    K: Ord;

impl<K, V> RwMutexMap<K, V>
where
    V: Clone,
    K: Ord,
{
    pub fn new() -> Self {
        Self(RwLock::new(BTreeMap::new()))
    }
}

impl<K, V> KeyValueStore<K, V> for RwMutexMap<K, V>
where
    K: Ord,
    V: Clone,
{
    type Err = Box<dyn Error>;

    fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        let mut map = self.0.write();
        let ins = map.insert(key, Arc::new(Mutex::new(value)));
        match ins {
            Some(v) => {
                let v = v.lock();
                Ok(Some(v.clone()))
            }
            None => Ok(None),
        }
    }

    fn remove(&self, key: &K) -> Result<Option<V>, Self::Err> {
        let mut map = self.0.write();
        let rm = map.remove(key);
        match rm {
            Some(v) => {
                let v = v.lock();
                Ok(Some(v.clone()))
            }
            None => Ok(None),
        }
    }

    fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        let map = self.0.read();
        Ok(map.contains(key)?)
    }

    fn mutate<F>(&self, key: &K, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>),
    {
        let map = self.0.read();
        let v = map.get(key);
        match v {
            Some(v) => {
                let mut v = v.lock();
                f(Some(&mut v));
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn for_each<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V)),
    {
        let map = self.0.read();
        map.iter().for_each(|(k, v)| {
            let v = v.lock();
            f((k, &*v));
        });

        Ok(())
    }

    fn for_each_mut<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)),
    {
        let map = self.0.read();
        map.iter().for_each(|(k, v)| {
            let mut v = v.lock();
            f((k, &mut *v));
        });

        Ok(())
    }

    fn inspect<F>(&self, key: &K, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>),
    {
        let map = self.0.read();
        let v = map.get(key);
        match v {
            Some(v) => {
                let v = v.lock();
                f(Some(&v));
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl<K, V> KeyValueStore<K, V> for RwLock<BTreeMap<K, V>>
where
    V: Clone,
    K: std::cmp::Ord,
{
    type Err = Box<dyn Error>;

    fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        let mut map = self.write();
        Ok(map.insert(key, value))
    }

    fn remove(&self, key: &K) -> Result<Option<V>, Self::Err> {
        let mut map = self.write();
        let v = map.remove(key);
        Ok(v)
    }

    fn mutate<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>),
    {
        let mut map = self.write();
        map.mutate(key, f)?;
        Ok(())
    }

    fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        let map = self.read();
        let b = map.contains(key)?;
        Ok(b)
    }

    fn for_each<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V)),
    {
        let map = self.read();
        map.iter().for_each(|v| f(v));
        Ok(())
    }

    fn for_each_mut<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)),
    {
        let mut map = self.write();
        map.iter_mut().for_each(|v| f(v));
        Ok(())
    }

    fn inspect<F>(&self, key: &K, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>),
    {
        let map = self.read();
        let b = map.get(key);
        f(b);
        Ok(())
    }
}
#[test]
fn test_rwmutexmap() {
    use crate::parking_lot::RwMutexMap;

    let kvstore = Arc::new(RwMutexMap::new());
    crate::test::test_impl(&kvstore);
}
