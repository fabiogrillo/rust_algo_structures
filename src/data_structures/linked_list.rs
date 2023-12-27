pub mod linked_list {
    // Define the Node struct
    #[derive(Clone)]
    pub struct Node<T>
    where
        T: Clone,
    {
        value: T,
        next: Option<Box<Node<T>>>,
        prev: Option<Box<Node<T>>>,
    }

    impl<T: Clone> Node<T>
    where
        T: Clone,
    {
        pub fn new(value: T) -> Self {
            Node {
                value,
                next: None,
                prev: None,
            }
        }
    }

    // define the LinkedList struct
    pub struct LinkedList<T: std::clone::Clone> {
        head: Option<Box<Node<T>>>,
        tail: Option<Box<Node<T>>>,
        len: usize,
    }

    impl<T: Clone> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList {
                head: None,
                tail: None,
                len: 0,
            }
        }

        // return wheter the linked list is empty or not
        pub fn is_empty(self) -> bool {
            match self.len {
                0 => return true,
                _ => return false,
            };
        }

        // return the size of the linked list
        pub fn len(self) -> usize {
            self.len
        }

        // append a node at the end of the linked list
        pub fn push_back(&mut self, value: T) {
            let mut new_node = Box::new(Node {
                value: value,
                prev: None,
                next: None,
            });

            match self.tail.take() {
                Some(mut old_tail) => {
                    old_tail.next = Some(new_node.clone());
                    new_node.prev = Some(old_tail);
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }

            self.len += 1;
        }

        // add an element first in the list
        pub fn push_front(&mut self, value: T) {
            let mut new_node = Box::new(Node {
                value: value,
                prev: None,
                next: None,
            });

            match self.head.take() {
                Some(mut old_head) => {
                    old_head.prev = Some(new_node.clone());
                    new_node.next = Some(old_head);
                    self.head = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }

            self.len += 1;
        }
    }

    // From here on there are some tests to test my self-implemented methods
    #[cfg(test)]
    mod tests {
        use std::collections::linked_list;

        use super::*;

        #[test]
        fn test_new_node() {
            let node = Node::new(5);
            assert_eq!(node.value, 5);
            assert!(node.prev.is_none());
            assert!(node.next.is_none());
        }

        #[test]
        fn test_new_linked_list() {
            let ll = LinkedList::<i32>::new();
            assert!(ll.head.is_none());
            assert!(ll.tail.is_none());
            assert_eq!(ll.len, 0);
        }

        #[test]
        fn test_push_back_linked_list() {
            let mut list = LinkedList::<i32>::new();

            list.push_back(5);
            assert_eq!(list.len, 1);
            assert_eq!(list.head.as_ref().unwrap().value, 5);
            assert_eq!(list.tail.as_ref().unwrap().value, 5);

            list.push_back(10);
            assert_eq!(list.len, 2);
            assert_eq!(list.head.as_ref().unwrap().value, 5);
            assert_eq!(list.tail.as_ref().unwrap().value, 10);
        }

        #[test]
        fn test_is_empty_linked_list() {
            let ll = LinkedList::<i32>::new();
            assert_eq!(ll.is_empty(), true);
        }

        #[test]
        fn test_linked_list_len() {
            let mut ll = LinkedList::<i32>::new();
            assert_eq!(ll.len, 0);
            ll.push_back(10);
            assert_eq!(ll.len, 1);
        }

        #[test]
        fn test_push_front_linked_list() {
            let mut ll = LinkedList::<i32>::new();
            ll.push_back(5);
            ll.push_front(2);
            assert_eq!(ll.head.as_ref().unwrap().value, 2);
            ll.push_back(3);
            let tail_node = ll.tail.as_ref().unwrap();
            let prev_node = tail_node.prev.as_ref().unwrap();
            assert_eq!(prev_node.value, 5);
            assert_eq!(ll.tail.as_ref().unwrap().value, 3);

        }
    }
}
