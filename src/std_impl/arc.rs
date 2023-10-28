use crate::traits::{KeyValueStore, MutKeyValueStore};
use std::{error::Error, sync::Arc};
impl<T, K, V> KeyValueStore<K, V> for Arc<T>
where
    T: KeyValueStore<K, V, Err = Box<dyn Error>>,
{
    type Err = Box<dyn Error>;

    fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        Ok(T::insert(&self, key, value)?)
    }

    fn remove(&self, key: &K) -> Result<Option<V>, Self::Err> {
        Ok(T::remove(&self, key)?)
    }

    fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        Ok(T::contains(&self, key)?)
    }

    fn mutate<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>),
    {
        Ok(T::mutate(self, key, f)?)
    }

    fn for_each<F: FnMut((&K, &V))>(&self, f: F) -> Result<(), Self::Err> {
        Ok(T::for_each(self, f)?)
    }

    fn for_each_mut<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)),
    {
        Ok(T::for_each_mut(self, f)?)
    }

    fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>),
    {
        Ok(T::inspect(self, key, f)?)
    }
}

impl<T, K, V> MutKeyValueStore<K, V> for Arc<T>
where
    T: KeyValueStore<K, V, Err = Box<dyn Error>>,
{
    type Err = Box<dyn Error>;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::Err> {
        Ok(T::insert(self, key, value)?)
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>, Self::Err> {
        Ok(T::remove(self, key)?)
    }

    fn contains(&self, key: &K) -> Result<bool, Self::Err> {
        Ok(T::contains(self, key)?)
    }

    fn mutate<F>(&mut self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>),
    {
        Ok(T::mutate(self, key, f)?)
    }

    fn for_each<F: FnMut((&K, &V))>(&self, f: F) -> Result<(), Self::Err> {
        Ok(T::for_each(self, f)?)
    }

    fn for_each_mut<F>(&mut self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)),
    {
        Ok(T::for_each_mut(&self, f)?)
    }

    fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>),
    {
        Ok(T::inspect(self, key, f)?)
    }
}
