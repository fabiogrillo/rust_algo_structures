pub mod hash_map {
    use std::{
        collections::{hash_map::DefaultHasher, LinkedList},
        hash::Hasher,
    };

    pub struct HashMap<K, V>
    where
        K: Clone,
        V: Clone,
    {
        buckets: Vec<LinkedList<(K, V)>>,
        size: usize,
    }

    impl<K: Eq + std::hash::Hash + Clone, V: Clone> HashMap<K, V> {
        pub fn new(size: usize) -> Self {
            Self {
                buckets: vec![LinkedList::new(); size],
                size,
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % self.size as u64) as usize;
            let bucket_list = &mut self.buckets[bucket];

            for &mut (ref e_key, ref mut e_value) in bucket_list.iter_mut() {
                if e_key == &key {
                    *e_value = value.clone();
                    return;
                }
            }

            bucket_list.push_back((key, value));
        }

        pub fn search(&self, key: &K) -> Option<&V> {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % self.size as u64) as usize;
            let bucket_list = &self.buckets[bucket];

            for &(ref e_key, ref e_value) in bucket_list.iter() {
                if e_key == key {
                    return Some(e_value);
                }
            }

            None
        }

        pub fn delete(&mut self, key: &K) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % self.size as u64) as usize;
            let bucket_list = &mut self.buckets[bucket];

            let mut i = 0;
            for (ref e_key, _) in bucket_list.iter() {
                if e_key == key {
                    break;
                }
                i += 1;
            }

            if i < bucket_list.len() {
                let mut rest = bucket_list.split_off(i);
                if i < rest.len() {
                    rest.pop_front();
                }
                bucket_list.append(&mut rest);
            }
        }

        pub fn contains_key(&self, key: &K) -> bool {
            self.search(key).is_some()
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.search(key)
        }

        pub fn clear(&mut self) {
            for bucket in &mut self.buckets {
                bucket.clear();
            }
        }

        pub fn len(&self) -> usize {
            self.buckets.iter().map(|bucket| bucket.len()).sum()
        }

        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.buckets
                .iter()
                .flat_map(|bucket| bucket.iter().map(|(key, _)| key))
        }

        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.buckets
                .iter()
                .flat_map(|bucket| bucket.iter().map(|(_, value)| value))
        }

        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.buckets
                .iter()
                .flat_map(|bucket| bucket.iter().map(|(key, value)| (key, value)))
        }

        pub fn resize(&mut self, new_size: usize) {
            let mut new_buckets = vec![LinkedList::new(); new_size];

            for bucket in &self.buckets {
                for &(ref key, ref value) in bucket.iter() {
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    let new_bucket = (hasher.finish() % new_size as u64) as usize;
                    new_buckets[new_bucket].push_back((key.clone(), value.clone()));
                }
            }

            self.buckets = new_buckets;
            self.size = new_size;
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn size(&self) -> usize {
            self.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let map: HashMap<i32, i32> = HashMap::new(10);
            assert_eq!(map.buckets.len(), 10);
            assert_eq!(map.size, 10);
        }

        #[test]
        fn test_insert() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            assert_eq!(map.buckets.iter().flatten().count(), 3);
        }

        #[test]
        fn test_search() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            assert_eq!(map.search(&1), Some(&2));
            assert_eq!(map.search(&3), Some(&4));
            assert_eq!(map.search(&5), Some(&6));
            assert_eq!(map.search(&7), None);
        }

        #[test]
        fn test_delete() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            map.delete(&1);
            assert_eq!(map.search(&1), None);

            map.delete(&3);
            assert_eq!(map.search(&3), None);
        }

        #[test]
        fn test_contains_key_and_get() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            assert!(map.contains_key(&1));
            assert_eq!(map.get(&1), Some(&2));

            assert!(map.contains_key(&3));
            assert_eq!(map.get(&3), Some(&4));

            assert!(map.contains_key(&5));
            assert_eq!(map.get(&5), Some(&6));

            assert!(!map.contains_key(&7));
            assert_eq!(map.get(&7), None);
        }

        #[test]
        fn test_clear() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            map.clear();

            assert_eq!(map.buckets.iter().flatten().count(), 0);
        }

        #[test]
        fn test_len_keys_values_iter() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            assert_eq!(map.len(), 3);

            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            assert_eq!(keys, vec![&1, &3, &5]);

            let mut values: Vec<_> = map.values().collect();
            values.sort();
            assert_eq!(values, vec![&2, &4, &6]);

            let mut pairs: Vec<_> = map.iter().collect();
            pairs.sort_by_key(|&(key, _)| *key);
            assert_eq!(pairs, vec![(&1, &2), (&3, &4), (&5, &6)]);
        }

        #[test]
        fn test_resize() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            map.resize(20);

            assert_eq!(map.size, 20);
            assert_eq!(map.len(), 3);
            assert_eq!(map.search(&1), Some(&2));
            assert_eq!(map.search(&3), Some(&4));
            assert_eq!(map.search(&5), Some(&6));
        }

        #[test]
        fn test_is_empty_and_size() {
            let mut map: HashMap<i32, i32> = HashMap::new(10);

            assert!(map.is_empty());
            assert_eq!(map.size(), 0);

            map.insert(1, 2);

            assert!(!map.is_empty());
            assert_eq!(map.size(), 1);
        }
    }
}
