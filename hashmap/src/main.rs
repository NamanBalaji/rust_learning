use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let mut map = OurMap::new();
    map.insert(1, 2);
    let val = map.get(1).expect("should not be None");

    assert!(*val == 2);

    let mut map2 = OurMap::new();
    map2.insert("xyz", 789);
    map2.insert("dasd", 2143);

    let val1 = map2.get("xyz").expect("should not be None");
    let val2 = map2.get("dasd").expect("should not be None");
    assert!(*val1 == 789);
    assert!(*val2 == 2143);
}

struct OurMap<K, V> {
    buckets: Vec<Vec<Option<(K, V)>>>,
}

const INITIAL_BUCKET_SIZE: usize = 5381;

impl<K: Hash + PartialEq, V> OurMap<K, V> {
    fn new() -> Self {
        let mut buckets = Vec::with_capacity(INITIAL_BUCKET_SIZE);
        for _ in 0..INITIAL_BUCKET_SIZE {
            buckets.push(Vec::new());
        }

        OurMap { buckets }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.get_index_for_key(&key);
        self.buckets[index].push(Some((key, value)));
    }

    fn get(&self, key: K) -> Option<&V> {
        let index = self.get_index_for_key(&key);
        let bucket = self
            .buckets
            .get(index)
            .expect("Index should not be out of range");
        for pair in bucket {
            match pair {
                Some((k, v)) => {
                    if *k == key {
                        return Some(v);
                    }
                }
                None => continue,
            }
        }

        None
    }

    fn get_index_for_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}
