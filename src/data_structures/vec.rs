pub mod vec {
    use core::panic;

    pub struct Vec<T> {
        buffer: Box<[T]>,
        len: usize,
    }

    impl<T: Clone + Default> Vec<T> {
        pub fn new() -> Self {
            Self {
                buffer: Box::new([]),
                len: 0,
            }
        }

        pub fn push(&mut self, el: T) {
            if self.len == self.buffer.len() {
                self.grow();
            }
            self.buffer[self.len] = el;
            self.len += 1;
        }

        fn grow(&mut self) {
            let new_capacity = self.buffer.len().max(1) * 2;
            let mut new_buffer = vec![T::default(); new_capacity].into_boxed_slice();
            for i in 0..self.len {
                new_buffer[i] = self.buffer[i].clone();
            }
            self.buffer = new_buffer;
        }

        // removes the last element from a vector and return it
        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                Some(self.buffer[self.len].clone())
            }
        }

        // insert an element at postion index within the vectore, shifting all elements after it to the right
        pub fn insert(&mut self, index: usize, item: T) {
            if index > self.len {
                panic!("index out of bounds");
            }
            if self.len == self.buffer.len() {
                self.grow();
            }
            for i in (index + 1..self.len + 1).rev() {
                self.buffer[i] = self.buffer[i - 1].clone();
            }
            self.buffer[index] = item;
            self.len += 1;
        }

        // removes and return the element at position index within the vector, shifting all elements after it to the left
        pub fn remove(&mut self, index: usize) -> T {
            if index >= self.len {
                panic!("index out of bounds");
            }
            let result = self.buffer[index].clone();
            for i in index..self.len - 1 {
                self.buffer[i] = self.buffer[i + 1].clone();
            }
            self.len -= 1;
            result
        }

        // returns a reference to the element at position index within the vector, or Non e if out of bounds
        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                None
            } else {
                Some(&self.buffer[index])
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_push_and_grow() {
            let mut vec = Vec::new();
            for i in 0..100 {
                vec.push(i);
            }
            assert_eq!(vec.len, 100);
            assert_eq!(vec.buffer.len(), 128); // buffer size should be next power of 2
            for i in 0..100 {
                assert_eq!(vec.buffer[i], i);
            }
        }

        #[test]
        fn test_pop() {
            let mut vec = Vec::new();
            vec.push(1);
            assert_eq!(vec.pop(), Some(1));
            assert_eq!(vec.pop(), None);
        }

        #[test]
        fn test_insert() {
            let mut vec = Vec::new();
            vec.insert(0, 1);
            vec.insert(1, 2);
            vec.insert(1, 3);
            assert_eq!(vec.len, 3);
            assert_eq!(vec.buffer[0], 1);
            assert_eq!(vec.buffer[1], 3);
            assert_eq!(vec.buffer[2], 2);
        }

        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn test_insert_out_of_bounds() {
            let mut vec = Vec::new();
            vec.insert(1, 1);
        }

        #[test]
        fn test_remove() {
            let mut vec = Vec::new();
            vec.push(1);
            vec.push(2);
            vec.push(3);
            assert_eq!(vec.remove(1), 2);
            assert_eq!(vec.len, 2);
            assert_eq!(vec.buffer[0], 1);
            assert_eq!(vec.buffer[1], 3);
        }

        #[test]
        fn test_get() {
            let mut vec = Vec::new();
            vec.push(1);
            assert_eq!(vec.get(0), Some(&1));
            assert_eq!(vec.get(1), None);
        }
    }
}
