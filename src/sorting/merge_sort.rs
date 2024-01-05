pub mod merge_sort {

    pub fn merge_sort<T: PartialOrd + Clone + Copy>(arr: &mut [T]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        let mut ret = arr.to_vec();

        merge(&arr[..mid], &arr[mid..], &mut ret[..]);

        arr.copy_from_slice(&ret);
    }

    fn merge<T: PartialOrd + Clone + Copy>(a: &[T], b: &[T], ret: &mut [T]) {
        let mut left = 0;
        let mut right = 0;
        let mut index = 0;

        while left < a.len() && right < b.len() {
            if a[left] <= b[right] {
                ret[index] = a[left].clone();
                index += 1;
                left += 1;
            } else {
                ret[index] = b[right].clone();
                index += 1;
                right += 1;
            }
        }

        if left < a.len() {
            ret[index..].copy_from_slice(&a[left..]);
        }
        if right < b.len() {
            ret[index..].copy_from_slice(&b[right..]);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_merge_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            merge_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            merge_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
