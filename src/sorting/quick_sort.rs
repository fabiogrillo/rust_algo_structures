pub mod quick_sort {
    pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len = arr.len();
        if len < 2 {
            return;
        }

        let pivot_index = partition(arr);
        quick_sort(&mut arr[0..pivot_index]);
        quick_sort(&mut arr[pivot_index + 1..]);
    }

    fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len / 2;
        arr.swap(pivot_index, len - 1);
        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_quick_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            quick_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            quick_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
