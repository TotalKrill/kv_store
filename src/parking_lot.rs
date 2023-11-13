use crate::traits::KeyValueStore;
use async_trait::async_trait;
use parking_lot::{Mutex, RwLock};
use std::{collections::BTreeMap, error::Error, sync::Arc};

/// Read write locked BTreeMap, that reduces WriteLocks by only using them when adding new keys, or removing keys,
/// But not when updating already existing values, or adding to an already added key
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

#[async_trait]
impl<K, V> KeyValueStore<K, V> for RwMutexMap<K, V>
where
    K: Ord + Send + Sync,
    V: Clone + Send + Sync,
{
    type Err = Box<dyn Error>;

    async fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        if self.contains(&key).await? {
            let mut old = None;
            self.get_mut(&key, |ov| {
                // just overwrite
                old = Some(ov.clone());
                *ov = value.clone();
            })
            .await?;

            Ok(old)
        } else {
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
    }

    async fn remove(&self, key: &K) -> Result<Option<V>, Self::Err> {
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

    async fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        let map = self.0.read();
        Ok(map.contains_key(key))
    }

    async fn get_mut<F>(&self, key: &K, mut f: F) -> Result<bool, Self::Err>
    where
        F: FnMut(&mut V) + Send,
    {
        let map = self.0.read();
        let v = map.get(key);
        match v {
            Some(v) => {
                let mut v = v.lock();
                f(&mut v);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn for_each<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V)) + Send,
    {
        let map = self.0.read();
        map.iter().for_each(|(k, v)| {
            let v = v.lock();
            f((k, &*v));
        });

        Ok(())
    }

    async fn for_each_mut<F>(&self, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)) + Send,
    {
        let map = self.0.read();
        map.iter().for_each(|(k, v)| {
            let mut v = v.lock();
            f((k, &mut *v));
        });

        Ok(())
    }

    async fn inspect<F>(&self, key: &K, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>) + Send,
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

#[async_trait]
impl<T, K, V> KeyValueStore<K, V> for Arc<T>
where
    T: KeyValueStore<K, V> + Send + Sync,
    K: Ord + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    type Err = <T as KeyValueStore<K, V>>::Err;

    async fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        Ok(T::insert(self, key, value).await?)
    }
    async fn remove(&self, key: &K) -> Result<Option<V>, Self::Err> {
        Ok(T::remove(self, key).await?)
    }
    async fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        Ok(T::contains(self, key).await?)
    }
    async fn get_mut<F>(&self, key: &K, f: F) -> Result<bool, Self::Err>
    where
        F: FnMut(&mut V) + Send,
    {
        Ok(T::get_mut(self, key, f).await?)
    }
    async fn for_each<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V)) + Send,
    {
        Ok(T::for_each(self, f).await?)
    }
    async fn for_each_mut<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)) + Send,
    {
        Ok(T::for_each_mut(self, f).await?)
    }
    async fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>) + Send,
    {
        Ok(T::inspect(self, key, f).await?)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_rwmutexmap() {
    use crate::parking_lot::RwMutexMap;

    let kvstore = Arc::new(RwMutexMap::new());
    crate::test::test_impl(&kvstore).await;
}
