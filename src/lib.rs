pub mod prelude {
    pub use crate::traits::{GetBTreeMap, GetHashMap, GetOwned, KeyValueStore, MutKeyValueStore};
}

pub mod traits;

pub mod std_impl;

pub mod parking_lot;

//mod itertest;

#[cfg(test)]
pub mod test {

    use crate::prelude::*;

    pub fn test_impl<T: KeyValueStore<usize, String>>(kvstore: &T) {
        kvstore.insert(1, String::from("hello")).ok();
        kvstore
            .inspect(&1, |v| assert_eq!(Some("hello"), v.map(|x| x.as_str())))
            .ok();

        kvstore.inspect(&2, |v| assert_eq!(None, v)).ok();

        // can use clone the value
        let mut s = String::new();
        kvstore
            .inspect(&1, |v| match v {
                Some(v) => s = v.clone(),
                None => assert!(true, "should be a value here"),
            })
            .ok();
        assert_eq!(s, "hello".to_string());

        kvstore
            .mutate(&1, |v| match v {
                Some(v) => {
                    // can mutate the value
                    *v = "world".to_string()
                }
                None => {}
            })
            .ok();

        kvstore
            .inspect(&1, |v| assert_eq!(Some("world"), v.map(|x| x.as_str())))
            .ok();
    }

    pub fn test_impl_mut<T: MutKeyValueStore<usize, String>>(mut kvstore: T) {
        kvstore.insert(1, String::from("hello")).ok();
        kvstore
            .inspect(&1, |v| assert_eq!(Some("hello"), v.map(|x| x.as_str())))
            .ok();

        kvstore.inspect(&2, |v| assert_eq!(None, v)).ok();

        // can use clone the value
        let mut s = String::new();
        kvstore
            .inspect(&1, |v| match v {
                Some(v) => s = v.clone(),
                None => assert!(true, "should be a value here"),
            })
            .ok();

        assert_eq!(s, "hello".to_string());

        kvstore
            .mutate(&1, |v| match v {
                Some(v) => {
                    // can mutate the value
                    *v = "world".to_string()
                }
                None => {}
            })
            .ok();

        kvstore
            .inspect(&1, |v| assert_eq!(Some("world"), v.map(|x| x.as_str())))
            .ok();
    }
}
