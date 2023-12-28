pub mod linked_list {
    use std::{
        cell::RefCell,
        fmt::{Debug, Display},
        rc::Rc,
    };

    // Define the Node struct
    #[derive(Clone)]
    pub struct Node<T: Clone> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
        prev: Option<Rc<RefCell<Node<T>>>>,
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
    pub struct LinkedList<T: Clone> {
        head: Option<Rc<RefCell<Node<T>>>>,
        tail: Option<Rc<RefCell<Node<T>>>>,
        len: usize,
    }

    impl<T: Clone + PartialEq + Debug + Display> LinkedList<T> {
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
            let new_node = Rc::new(RefCell::new(Node {
                value: value.clone(),
                prev: None,
                next: None,
            }));

            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(old_tail.clone());
                    self.tail = Some(new_node.clone());
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node.clone());
                }
            }

            self.len += 1;
        }

        // add an element first in the list
        pub fn push_front(&mut self, value: T) {
            let new_node = Rc::new(RefCell::new(Node {
                value: value.clone(),
                prev: None,
                next: None,
            }));

            match self.head.take() {
                Some(old_head) => {
                    old_head.borrow_mut().prev = Some(new_node.clone());
                    new_node.borrow_mut().next = Some(old_head.clone());
                    self.head = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node.clone());
                }
            }

            self.len += 1;
        }

        // removes the last element from a list and returns it, or None if it is empty
        pub fn pop_back(&mut self) -> Option<T> {
            match self.tail.take() {
                Some(old_tail) => {
                    self.len -= 1;
                    if let Some(prev_node) = old_tail.borrow_mut().prev.take() {
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node);
                    } else {
                        self.head = None;
                    }
                    Some(old_tail.borrow().value.clone())
                }
                None => None,
            }
        }

        // removes the first element from a list and returns it, or None if it is empty
        pub fn pop_front(&mut self) -> Option<T> {
            match self.head.take() {
                Some(old_head) => {
                    self.len -= 1;
                    if let Some(next_node) = old_head.borrow_mut().next.take() {
                        next_node.borrow_mut().prev = None;
                        self.head = Some(next_node);
                    } else {
                        self.tail = None;
                    }
                    Some(old_head.borrow().value.clone())
                }
                None => None,
            }
        }

        // returns true if the linked list contains an element equal to the given value
        pub fn contains(&self, value: T) -> bool {
            let mut current = self.head.clone();

            while let Some(node) = current {
                if node.borrow().value == value {
                    return true;
                }
                current = node.borrow().next.clone();
            }

            false
        }

        // print the list like an array
        pub fn to_string(&self) -> String {
            let mut current = self.head.clone();
            let mut list_string = "[".to_string();

            while let Some(node) = current {
                list_string.push_str(&format!("{:?}", node.borrow().value));
                current = node.borrow().next.clone();
                if current.is_some() {
                    list_string.push_str(", ");
                }
            }

            list_string.push_str("]");
            list_string
        }

        // removes the element at the given index and returns it
        pub fn remove(&mut self, index: usize) -> Option<T> {
            if index >= self.len {
                return None;
            }

            let mut current = self.head.clone();
            for _ in 0..index {
                current = current.unwrap().borrow().next.clone();
            }

            let node = current.unwrap();
            let prev_node = node.borrow().prev.clone();
            let next_node = node.borrow().next.clone();

            match (&prev_node, &next_node) {
                (Some(prev), Some(next)) => {
                    prev.borrow_mut().next = Some(next.clone());
                    next.borrow_mut().prev = Some(prev.clone());
                }
                (None, Some(next)) => {
                    next.borrow_mut().prev = None;
                    self.head = Some(next.clone());
                }
                (Some(prev), None) => {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev.clone());
                }
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.len -= 1;
            let x = Some(node.borrow().value.clone());
            x
        }

        // append method to move all elements from other to the end of the list
        pub fn append(&mut self, other: &mut Self) {
            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = other.head.take();
                    self.tail = other.tail.take();
                }
                None => {
                    self.head = other.head.take();
                    self.tail = other.tail.take();
                }
            }
            self.len += other.len;
            other.len = 0;
        }

        // clear method to flush all the elements in the linked list
        pub fn clear(&mut self) {
            self.head = None;
            self.tail = None;
            self.len = 0;
        }
    }

    // From here on there are some tests to test my self-implemented methods
    #[cfg(test)]
    mod tests {
        //use std::collections::linked_list;

        use std::{cell::RefCell, rc::Rc};

        use crate::data_structures::linked_list::linked_list::{LinkedList, Node};

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
            assert_eq!(list.head.as_ref().unwrap().borrow().value, 5);
            assert_eq!(list.tail.as_ref().unwrap().borrow().value, 5);

            list.push_back(10);
            assert_eq!(list.len, 2);
            assert_eq!(list.head.as_ref().unwrap().borrow().value, 5);
            assert_eq!(list.tail.as_ref().unwrap().borrow().value, 10);
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
            assert_eq!(ll.head.as_ref().unwrap().borrow().value, 2);
            ll.push_back(3);
            let tail_node = ll.tail.as_ref().unwrap();
            let binding = tail_node.borrow();
            let prev_node = binding.prev.as_ref().unwrap();
            assert_eq!(prev_node.borrow().value, 5);
            assert_eq!(ll.tail.as_ref().unwrap().borrow().value, 3);
        }

        #[test]
        fn test_pop_back_linked_list() {
            let mut ll = LinkedList::<i32>::new();
            ll.push_back(5);
            ll.push_back(10);

            assert_eq!(ll.pop_back(), Some(10));
            assert_eq!(ll.len, 1);
            assert_eq!(ll.tail.as_ref().unwrap().borrow().value, 5);
            assert_eq!(ll.pop_back(), Some(5));
            assert_eq!(ll.len, 0);
            assert!(ll.tail.is_none());
            assert_eq!(ll.pop_back(), None);
        }

        #[test]
        fn test_pop_front_linked_list() {
            let mut ll = LinkedList::<i32>::new();
            ll.push_front(5);
            ll.push_front(10);

            assert_eq!(ll.pop_front(), Some(10));
            assert_eq!(ll.len, 1);
            assert_eq!(ll.head.as_ref().unwrap().borrow().value, 5);
            assert_eq!(ll.pop_front(), Some(5));
            assert_eq!(ll.len, 0);
            assert!(ll.tail.is_none());
            assert_eq!(ll.pop_front(), None);
        }

        #[test]
        fn test_contains() {
            let mut list = LinkedList::<i32>::new();

            list.push_back(5);
            list.push_back(10);
            list.push_back(20);
            assert!(list.contains(5));
            assert!(list.contains(10));
            //println!("{:?}", list.to_string());
            assert!(!list.contains(15));
        }

        #[test]
        fn test_remove() {
            let mut list = LinkedList::<i32>::new();

            list.push_back(5);
            list.push_back(10);
            list.push_back(15);

            assert_eq!(list.remove(1), Some(10));
            assert_eq!(list.len, 2);
            assert_eq!(list.head.as_ref().unwrap().borrow().value, 5);
            assert_eq!(list.tail.as_ref().unwrap().borrow().value, 15);

            assert_eq!(list.remove(0), Some(5));
            assert_eq!(list.len, 1);
            assert_eq!(list.head.as_ref().unwrap().borrow().value, 15);
            assert_eq!(list.tail.as_ref().unwrap().borrow().value, 15);

            assert_eq!(list.remove(0), Some(15));
            assert_eq!(list.len, 0);
            assert!(list.head.is_none());
            assert!(list.tail.is_none());

            assert_eq!(list.remove(0), None);
        }

        #[test]
        fn test_clear() {
            let mut list = LinkedList {
                head: Some(Rc::new(RefCell::new(Node::new(1)))),
                tail: Some(Rc::new(RefCell::new(Node::new(2)))),
                len: 2,
            };
            list.clear();
            assert!(list.head.is_none());
            assert!(list.tail.is_none());
            assert_eq!(list.len, 0);
        }

        #[test]
        fn test_append() {
            let mut list1 = LinkedList {
                head: Some(Rc::new(RefCell::new(Node::new(1)))),
                tail: Some(Rc::new(RefCell::new(Node::new(2)))),
                len: 2,
            };
            let mut list2 = LinkedList {
                head: Some(Rc::new(RefCell::new(Node::new(3)))),
                tail: Some(Rc::new(RefCell::new(Node::new(4)))),
                len: 2,
            };
            list1.append(&mut list2);
            assert_eq!(list1.len, 4);
            assert_eq!(list2.len, 0);
        }
    }
}
