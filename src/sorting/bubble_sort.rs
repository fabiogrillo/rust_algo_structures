pub mod bubble_sort {
    pub fn bubble_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
        let len = arr.len();
        for _ in 0..len {
            for j in 1..len {
                if arr[j - 1] > arr[j] {
                    arr.swap(j - 1, j);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            bubble_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            bubble_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
