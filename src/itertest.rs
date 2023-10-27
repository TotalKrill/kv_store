pub trait Iterable<'a, 'b, 'c, K: 'a, V: 'b> {
    fn iterate(&'c self) -> impl Iterator<Item = (&'a K, &'b V)>;
}

impl<'a, 'b, 'c, K, V> Iterable<'a, 'b, 'c, K, V> for BTreeMap<K, V>
where
    K: 'a,
    V: 'b,
    'c: 'a + 'b,
    'b: 'a,
    'a: 'b,
{
    fn iterate(&'c self) -> impl Iterator<Item = (&'a K, &'b V)> {
        self.iter()
    }
}

// just test to make a trait to be able to use the .iter() function on stuff
#[test]
fn iterate() {
    use crate::Iterable;
    let mut map = BTreeMap::new();
    map.insert(1, 4);
    map.insert(2, 3);
    map.insert(3, 2);
    map.insert(4, 1);
    let newmap: BTreeMap<usize, String> = map
        .iterate()
        .map(|(k, v)| (k.clone(), v.to_string()))
        .collect();
}
