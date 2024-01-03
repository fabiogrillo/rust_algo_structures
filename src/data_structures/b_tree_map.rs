pub mod b_tree_map {
    use std::cmp::Ord;

    // Define the BTreeMap struct
    pub struct BTreeMap<K: Ord, V> {
        root: Option<Box<Node<K, V>>>,
        b: usize,
    }

    // Define the Node struct
    struct Node<K: Ord, V> {
        keys: Vec<K>,
        vals: Vec<V>,
        children: Vec<Box<Node<K, V>>>,
    }

    // Implement the new function for BTreeMap
    impl<K: Ord, V> BTreeMap<K, V> {
        pub fn new(b: usize) -> Self {
            Self { root: None, b }
        }

        pub fn insert(&mut self, key: K, val: V) {
            match self.root {
                Some(ref mut root) => {
                    root.insert(key, val);
                }
                None => {
                    let mut root = Node::new();
                    root.keys.push(key);
                    root.vals.push(val);
                    self.root = Some(Box::new(root));
                }
            }
        }

        pub fn search(&self, key: &K) -> Option<&V> {
            match &self.root {
                Some(root) => root.search(key),
                None => None,
            }
        }

        pub fn delete(&mut self, key: &K) {
            if let Some(ref mut root) = self.root {
                root.delete(key);
            }
        }

        pub fn pre_order_traversal(&self) -> Vec<(&K, &V)> {
            match &self.root {
                Some(root) => root.pre_order_traversal(),
                None => Vec::new(),
            }
        }

        pub fn in_order_traversal(&self) -> Vec<(&K, &V)> {
            match &self.root {
                Some(root) => root.in_order_traversal(),
                None => Vec::new(),
            }
        }

        pub fn post_order_traversal(&self) -> Vec<(&K, &V)> {
            match &self.root {
                Some(root) => root.post_order_traversal(),
                None => Vec::new(),
            }
        }
    }

    // Implement the new function for Node
    impl<K: Ord, V> Node<K, V> {
        fn new() -> Self {
            Self {
                keys: Vec::new(),
                vals: Vec::new(),
                children: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, val: V) {
            let idx = self.keys.binary_search(&key).unwrap_or_else(|x| x);
            self.keys.insert(idx, key);
            self.vals.insert(idx, val);
        }

        fn search(&self, key: &K) -> Option<&V> {
            match self.keys.binary_search(key) {
                Ok(idx) => Some(&self.vals[idx]),
                Err(idx) => {
                    if idx < self.children.len() {
                        self.children[idx].search(key)
                    } else {
                        None
                    }
                }
            }
        }

        fn delete(&mut self, key: &K) {
            if let Ok(idx) = self.keys.binary_search(key) {
                self.keys.remove(idx);
                self.vals.remove(idx);
            } else {
                for child in &mut self.children {
                    child.delete(key);
                }
            }
        }

        fn pre_order_traversal(&self) -> Vec<(&K, &V)> {
            let mut result = Vec::new();
            for i in 0..self.keys.len() {
                result.push((&self.keys[i], &self.vals[i]));
                if i < self.children.len() {
                    result.extend(self.children[i].pre_order_traversal());
                }
            }
            result
        }

        fn in_order_traversal(&self) -> Vec<(&K, &V)> {
            let mut result = Vec::new();
            for i in 0..self.keys.len() {
                if i < self.children.len() {
                    result.extend(self.children[i].in_order_traversal());
                }
                result.push((&self.keys[i], &self.vals[i]));
            }
            if self.children.len() > self.keys.len() {
                result.extend(self.children.last().unwrap().in_order_traversal());
            }
            result
        }

        fn post_order_traversal(&self) -> Vec<(&K, &V)> {
            let mut result = Vec::new();
            for i in 0..self.keys.len() {
                if i < self.children.len() {
                    result.extend(self.children[i].post_order_traversal());
                }
                result.push((&self.keys[i], &self.vals[i]));
            }
            if self.children.len() > self.keys.len() {
                result.extend(self.children.last().unwrap().post_order_traversal());
            }
            result
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let map: BTreeMap<i32, i32> = BTreeMap::new(2);
            assert!(map.root.is_none());
            assert_eq!(map.b, 2);
        }

        #[test]
        fn test_insert() {
            let mut map: BTreeMap<i32, i32> = BTreeMap::new(2);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            let root = map.root.as_ref().unwrap();
            assert_eq!(root.keys, vec![1, 3, 5]);
            assert_eq!(root.vals, vec![2, 4, 6]);
        }

        #[test]
        fn test_search() {
            let mut map: BTreeMap<i32, i32> = BTreeMap::new(2);
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
            let mut map: BTreeMap<i32, i32> = BTreeMap::new(2);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            map.delete(&1);
            assert_eq!(map.search(&1), None);

            map.delete(&3);
            assert_eq!(map.search(&3), None);
        }

        #[test]
        fn test_traversal() {
            let mut map: BTreeMap<i32, i32> = BTreeMap::new(2);
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);

            assert_eq!(
                map.pre_order_traversal(),
                vec![(&1, &2), (&3, &4), (&5, &6)]
            );
            assert_eq!(map.in_order_traversal(), vec![(&1, &2), (&3, &4), (&5, &6)]);
            assert_eq!(
                map.post_order_traversal(),
                vec![(&1, &2), (&3, &4), (&5, &6)]
            );
        }
    }
}
