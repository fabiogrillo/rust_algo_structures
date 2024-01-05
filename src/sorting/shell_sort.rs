pub mod shell_sort {
    pub fn shell_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len = arr.len();
        let mut h = 1;
        while h < len / 3 {
            h = 3 * h + 1; // knuth sequence
        }

        while h >= 1 {
            for i in h..len {
                let mut j = i;
                while j >= h && arr[j] < arr[j - h] {
                    arr.swap(j, j - h);
                    j -= h;
                }
            }
            h /= 3;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_shell_sort() {
            let mut arr = [5, 2, 4, 1, 3];
            shell_sort(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);

            let mut arr = ['b', 'a', 'd', 'c', 'e'];
            shell_sort(&mut arr);
            assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
        }
    }
}
