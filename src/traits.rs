use std::collections::{BTreeMap, HashMap};

/// trait to give the most basic of key value store functionality to immutable objects
pub trait KeyValueStore<K, V> {
    type Err;
    /// Inserts a key-value pair into the map.

    /// If the map did not have this key present, None is returned.

    /// If the map did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated, though; this matters for types that can be == without
    /// being identical.
    fn insert(&self, key: K, value: V) -> Result<Option<V>, Self::Err>;

    /// Removes an entry from the map, returning the key and value if they existed in the map.
    fn remove(&self, key: &K) -> Result<Option<V>, Self::Err>;

    /// Checks if the map contains a specific key.
    fn contains(&self, key: &K) -> Result<bool, Self::Err>;

    /// Lets you run a function on the specified key
    fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>);

    /// runs a function that can mutate the value if it exists
    fn mutate<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>);

    /// mutates the value if it exists with the fiven FnMut, or inserts the given value
    fn mutate_or_insert<F>(&self, key: K, mut f: F, value: V) -> Result<(), Self::Err>
    where
        F: FnMut(&mut V),
    {
        let v = self.contains(&key)?;
        if v {
            let newf = |m: Option<&mut V>| match m {
                Some(m) => f(m),
                _ => {}
            };
            self.mutate(&key, newf)?;
        } else {
            self.insert(key, value)?;
        }
        Ok(())
    }

    /// runs the method on each of the values in the storage
    fn for_each<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V));

    /// runs the method on each of the values in the storage
    fn for_each_mut<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &mut V));
}

/// trait to give the most basic of key value storage functionality,
/// when having ownership
pub trait MutKeyValueStore<K, V> {
    type Err;
    /// Inserts a key-value pair into the map.

    /// If the map did not have this key present, None is returned.

    /// If the map did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated, though; this matters for types that can be == without
    /// being identical.
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::Err>;

    fn remove(&mut self, key: &K) -> Result<Option<V>, Self::Err>;

    // fn get(&self, key: &Key) -> Result<Option<&Value>, Self::Err>;
    fn mutate<F>(&mut self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&mut V>);

    /// This is what replaces the .get method, in the way that it allows
    fn inspect<F>(&self, key: &K, f: F) -> Result<(), Self::Err>
    where
        F: FnMut(Option<&V>);

    fn contains(&self, key: &K) -> Result<bool, Self::Err>;

    /// mutates the value if it exists with the fiven FnMut, or inserts the given value
    fn mutate_or_insert<F>(&mut self, key: K, mut f: F, value: V) -> Result<(), Self::Err>
    where
        F: FnMut(&mut V),
    {
        let v = self.contains(&key)?;
        if v {
            let newf = |m: Option<&mut V>| match m {
                Some(m) => f(m),
                _ => {}
            };
            self.mutate(&key, newf)?;
        } else {
            self.insert(key, value)?;
        }
        Ok(())
    }

    fn for_each<F>(&self, f: F) -> Result<(), Self::Err>
    where
        F: FnMut((&K, &V));

    fn for_each_mut<F: FnMut((&K, &mut V))>(&mut self, f: F) -> Result<(), Self::Err>;
}

/// Trait that enables to get a cloned value for the given key
pub trait GetOwned<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    fn get_owned(&self, key: &K) -> Result<Option<V>, Self::Err>;
}

impl<T, K, V> GetOwned<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>>,
    V: Clone,
{
    type Err = Box<dyn std::error::Error>;
    fn get_owned(&self, key: &K) -> Result<Option<V>, Self::Err> {
        let mut outer = None;
        self.inspect(key, |v| match v {
            Some(v) => outer = Some(v.clone()),
            None => outer = None,
        })?;
        Ok(outer)
    }
}
/// Trait that enables to get a cloned value for the given key
pub trait GetBTreeMap<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    fn btreemap(&self) -> Result<BTreeMap<K, V>, Self::Err>;
}

impl<T, K, V> GetBTreeMap<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>>,
    V: Clone,
    K: Clone + Ord,
{
    type Err = Box<dyn std::error::Error>;
    fn btreemap(&self) -> Result<BTreeMap<K, V>, Self::Err> {
        let mut map = BTreeMap::new();
        self.for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        })?;
        Ok(map)
    }
}
/// Trait that enables to get a cloned value for the given key
pub trait GetHashMap<K, V> {
    type Err;
    /// Gets a owned clone of the data in the map
    fn hashmap(&self) -> Result<HashMap<K, V>, Self::Err>;
}

impl<T, K, V> GetHashMap<K, V> for T
where
    T: KeyValueStore<K, V, Err = Box<dyn std::error::Error>>,
    V: Clone,
    K: Clone + std::hash::Hash + Eq,
{
    type Err = Box<dyn std::error::Error>;
    fn hashmap(&self) -> Result<HashMap<K, V>, Self::Err> {
        let mut map: HashMap<K, V> = Default::default();
        self.for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        })?;
        Ok(map)
    }
}
