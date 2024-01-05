pub mod insertion_sort {
    pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len = arr.len();
        for i in 1..len {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }
            arr[j] = key;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insertion_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            insertion_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            insertion_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
