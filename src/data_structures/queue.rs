pub mod queue {

    // a double-ended queu implemented with a growable ring buffer
    // its elements are not necessarily contiguous in memory
    pub struct Queue<T> {
        head: usize,
        len: usize,
        buffer: Box<[T]>,
    }

    impl<T: Clone + Default + std::fmt::Display> Queue<T> {
        pub fn new() -> Self {
            Self {
                head: 0,
                len: 0,
                buffer: Box::new([]),
            }
        }

        // appends an element to the back of the queue
        pub fn push_back(&mut self, value: T) {
            if self.is_full() {
                self.grow();
            }
            let index = (self.head + self.len) % self.buffer.len();
            self.buffer[index] = value;
            self.len += 1;
        }

        // appends an element to the back of the queue
        pub fn push_front(&mut self, value: T) {
            if self.is_full() {
                self.grow();
            }
            if self.len > 0 {
                self.head = (self.head + self.buffer.len() - 1) % self.buffer.len();
            }
            self.buffer[self.head] = value;
            self.len += 1;
        }

        // removes the last element from the queue and returns it or None if it is empty
        pub fn pop_back(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let index = (self.head + self.len - 1) % self.buffer.len();
                self.len -= 1;
                Some(self.buffer[index].clone())
            }
        }

        //removes the first element from the queue and return it or None if it is empty
        pub fn pop_front(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let value = self.buffer[self.head].clone();
                self.head = (self.head + 1) % self.buffer.len();
                self.len -= 1;
                Some(value)
            }
        }

        // return true if the buffer is full
        pub fn is_full(&self) -> bool {
            self.len == self.buffer.len()
        }

        // return true if the queu is empty
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        fn grow(&mut self) {
            let new_capacity = self.buffer.len().max(1) * 2;
            let mut new_buffer = vec![T::default(); new_capacity].into_boxed_slice();
            for i in 0..self.len {
                new_buffer[i] = self.buffer[i].clone();
            }
            self.buffer = new_buffer;
        }

        pub fn to_string(&self) -> String {
            let mut result = String::new();
            for i in 0..self.len {
                let index = (self.head + i) % self.buffer.len();
                result.push_str(&self.buffer[index].to_string());
                if i < self.len - 1 {
                    result.push_str("]");
                }
            }
            result
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_queue() {
            let queue = Queue::<i32>::new();
            assert_eq!(queue.len, 0);
        }

        #[test]
        fn test_push_back_queue() {
            let mut queue = Queue::<i32>::new();
            queue.push_back(5);
            queue.push_back(10);
            assert_eq!(queue.len, 2);
            assert_eq!(queue.buffer[1], 10);
        }

        #[test]
        fn test_push_front_queue() {
            let mut queue = Queue::<i32>::new();
            queue.push_front(4);
            assert_eq!(queue.buffer[queue.head], 4);
            queue.push_front(8);
            assert_eq!(queue.buffer[queue.head], 8);
            assert_eq!(queue.len, 2);
        }

        #[test]
        fn test_pop_back() {
            let mut queue = Queue {
                head: 0,
                len: 5,
                buffer: Box::new([1, 2, 3, 4, 5]),
            };
            assert_eq!(queue.pop_back(), Some(5));
            assert_eq!(queue.len, 4);
        }

        #[test]
        fn test_pop_front() {
            let mut queue = Queue {
                head: 0,
                len: 5,
                buffer: Box::new([1, 2, 3, 4, 5]),
            };
            assert_eq!(queue.pop_front(), Some(1));
            assert_eq!(queue.len, 4);
            assert_eq!(queue.head, 1);
        }
    }
}
