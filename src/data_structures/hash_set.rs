pub mod hash_set {
    pub struct HashSet<T> {
        storage: Vec<T>,
    }

    impl<T: PartialEq + Clone> HashSet<T> {
        pub fn new() -> Self {
            HashSet {
                storage: Vec::new(),
            }
        }

        pub fn insert(&mut self, key: T) {
            if !self.contains(&key) {
                self.storage.push(key);
            }
        }

        pub fn contains(&self, key: &T) -> bool {
            self.storage.contains(key)
        }

        pub fn remove(&mut self, key: &T) {
            self.storage.retain(|x| x != key);
        }

        pub fn clear(&mut self) {
            self.storage.clear();
        }

        pub fn len(&self) -> usize {
            self.storage.len()
        }

        pub fn is_empty(&self) -> bool {
            self.storage.is_empty()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.storage.iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::HashSet;

        #[test]
        fn test_hash_set() {
            let mut set = HashSet::new();
            set.insert("element");
            assert!(set.contains(&"element"));
            set.remove(&"element");
            assert!(!set.contains(&"element"));
        }

        #[test]
        fn test_hash_set_simple_methods() {
            let mut set = HashSet::new();
            assert!(set.is_empty());
            set.insert("element");
            assert_eq!(set.len(), 1);
            assert!(set.contains(&"element"));
            set.remove(&"element");
            assert!(!set.contains(&"element"));
            assert!(set.is_empty());
            set.insert("another element");
            assert_eq!(set.len(), 1);
            set.clear();
            assert!(set.is_empty());
        }
    }
}
