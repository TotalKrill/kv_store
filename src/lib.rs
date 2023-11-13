pub mod prelude {
    pub use crate::traits::{GetBTreeMap, GetHashMap, GetOwned, KeyValueStore};
}

pub mod traits;

#[cfg(feature = "impl")]
pub mod parking_lot;

#[cfg(test)]
pub mod test {

    use crate::prelude::*;

    pub async fn test_impl<T: KeyValueStore<usize, String>>(kvstore: &T) {
        kvstore.insert(1, String::from("hello")).await.ok();
        kvstore
            .inspect(&1, |v| assert_eq!(Some("hello"), v.map(|x| x.as_str())))
            .await
            .ok();

        kvstore.inspect(&2, |v| assert_eq!(None, v)).await.ok();

        // can use clone the value
        let mut s = String::new();
        kvstore
            .inspect(&1, |v| match v {
                Some(v) => s = v.clone(),
                None => assert!(true, "should be a value here"),
            })
            .await
            .ok();
        assert_eq!(s, "hello".to_string());

        kvstore
            .get_mut(&1, |v| {
                // can mutate the value
                *v = "world".to_string();
            })
            .await
            .ok();

        kvstore
            .inspect(&1, |v| assert_eq!(Some("world"), v.map(|x| x.as_str())))
            .await
            .ok();
    }
}
