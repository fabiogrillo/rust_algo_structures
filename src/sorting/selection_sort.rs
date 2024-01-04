pub mod selection_sort {
    pub fn selection_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut smallest = i;
            for j in (i + 1)..len {
                if arr[j] < arr[smallest] {
                    smallest = j;
                }
            }
            arr.swap(i, smallest);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_selection_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            selection_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            selection_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
