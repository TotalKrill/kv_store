use async_trait::async_trait;
use std::collections::{BTreeMap, HashMap};

/// trait to give the most basic of key value store functionality to immutable objects
#[async_trait]
pub trait KeyValueStore<K, V>
where
    K: Send + Sync,
    V: Send,
{
    type Err;
    /// Inserts a key-value pair into the map.

    /// If the map did not have this key present, None is returned.

    /// If the map did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated, though; this matters for types that can be == without
    /// being identical.
    async fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err>;

    /// Removes an entry from the map, returning the key and value if they existed in the map.
    async fn remove(&self, key: &K) -> Result<Option<V>, Self::Err>;

    /// Checks if the map contains a specific key.
    async fn contains(&self, key: &K) -> Result<bool, Self::Err>;

    /// Lets you run a function on the specified key
    async fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>) + Send;

    /// runs a function that can mutate the value if it exists
    // returns true, if the value was mutated
    async fn get_mut<F>(&self, key: &K, f: F) -> Result<bool, Self::Err>
    where
        F: FnMut(&mut V) + Send;

    /// mutates the value if it exists with the fiven FnMut, or inserts default, then mutates it
    async fn get_mut_or_default<F>(&self, key: K, mut f: F) -> Result<(), Self::Err>
    where
        F: FnMut(&mut V) + Send,
        K: 'async_trait,
        V: 'async_trait + Default,
    {
        let v = self.contains(&key).await?;
        if v {
            self.get_mut(&key, f).await?;
        } else {
            let mut value = Default::default();
            f(&mut value);
            self.insert(key, value).await?;
        }
        Ok(())
    }

    /// runs the method on each of the values in the storage
    async fn for_each<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V)) + Send;

    /// runs the method on each of the values in the storage
    async fn for_each_mut<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V)) + Send;
}

/// Trait that enables to get a cloned value for the given key
#[async_trait]
pub trait GetOwned<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    async fn get_owned(&self, key: &K) -> Result<Option<V>, Self::Err>;
}

#[async_trait]
impl<T, K, V> GetOwned<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>> + Send + Sync,
    K: Send + Sync,
    V: Clone + Send,
{
    type Err = Box<dyn std::error::Error>;
    async fn get_owned(&self, key: &K) -> Result<Option<V>, Self::Err> {
        let mut outer = None;
        self.inspect(key, |v| match v {
            Some(v) => outer = Some(v.clone()),
            None => outer = None,
        })
        .await?;
        Ok(outer)
    }
}
/// Trait that enables to get a cloned value for the given key
#[async_trait]
pub trait GetBTreeMap<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    async fn btreemap(&self) -> Result<BTreeMap<K, V>, Self::Err>;
}

#[async_trait]
impl<T, K, V> GetBTreeMap<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>> + Send + Sync,
    V: Clone + Send,
    K: Clone + Ord + Send + Sync,
{
    type Err = Box<dyn std::error::Error>;
    async fn btreemap(&self) -> Result<BTreeMap<K, V>, Self::Err> {
        let mut map = BTreeMap::new();
        self.for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        })
        .await?;
        Ok(map)
    }
}
/// Trait that enables to get a cloned value for the given key
#[async_trait]
pub trait GetHashMap<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    async fn hashmap(&self) -> Result<HashMap<K, V>, Self::Err>;
}

#[async_trait]
impl<T, K, V> GetHashMap<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>> + Send + Sync,
    V: Clone + Send,
    K: Clone + std::hash::Hash + Eq + Send + Sync,
{
    type Err = Box<dyn std::error::Error>;
    async fn hashmap(&self) -> Result<HashMap<K, V>, Self::Err> {
        let mut map: HashMap<K, V> = Default::default();
        self.for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        })
        .await?;
        Ok(map)
    }
}
