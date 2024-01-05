pub mod heap_sort {
    pub fn heap_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in (0..len / 2).rev() {
            sift_down(arr, i, len);
        }
        for i in (0..len).rev() {
            arr.swap(0, i);
            sift_down(arr, 0, i);
        }
    }

    fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
        let mut root = start;
        loop {
            let mut child = root * 2 + 1;
            if child >= end {
                break;
            }
            if child + 1 < end && arr[child] < arr[child + 1] {
                child += 1;
            }
            if arr[root] < arr[child] {
                arr.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_heap_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            heap_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            heap_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
