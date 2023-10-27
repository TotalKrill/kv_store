mod arc;

use std::convert::Infallible;

macro_rules! simple_mutmap_impl {
    ($map:ty, $($bounds:ident +)*) => {
        impl<K, V> MutKeyValueStore<K, V> for $map
        where
            K: $($bounds +)*,
        {
            type Err = Infallible;

            fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::Err> {
                Ok(self.insert(key, value))
            }

            fn remove(&mut self, key: &K) -> Result<Option<V>, Self::Err> {
                Ok(self.remove(key))
            }

            fn mutate<F>(&mut self, key: &K, mut f: F) -> Result<(), Infallible>
            where
                F: FnMut(Option<&mut V>),
            {
                let v = self.get_mut(key);
                f(v);
                Ok(())
            }

            fn inspect<F>(&self, key: &K, mut f: F) -> Result<(), Self::Err>
            where
                F: FnMut(Option<&V>),
            {
                let v = self.get(key);
                f(v);
                Ok(())
            }

            fn contains(&self, key: &K) -> Result<bool, Self::Err> {
                Ok(self.contains_key(key))
            }

            fn for_each<F>(&self, mut f: F) -> Result<(), Self::Err>
            where
                F: FnMut((&K, &V)),
            {
                self.iter().for_each(|v| f(v));
                Ok(())
            }

            fn for_each_mut<F>(&mut self, mut f: F) -> Result<(), Self::Err>
            where
                F: FnMut((&K, &mut V)),
            {
                self.iter_mut().for_each(|v| f(v));
                Ok(())
            }
        }
    }
}

use crate::MutKeyValueStore;
use std::cmp::Ord;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

simple_mutmap_impl!(BTreeMap<K,V>, Ord +);
simple_mutmap_impl!(HashMap<K,V>, Hash + Eq +);

#[test]
fn test_btreemap() {
    let kvstore = BTreeMap::new();
    crate::test::test_impl_mut(kvstore);
}
