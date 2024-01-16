pub fn bubble_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in 0..arr.len() - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut a1: Vec<i32> = vec![];
        bubble_sort(&mut a1);

        let mut a2: Vec<i32> = vec![1];
        bubble_sort(&mut a2);
        assert_eq!(a2[0], 1);

        let mut a3: Vec<i32> = vec![1, 2];
        bubble_sort(&mut a3);
        assert_eq!(a3, vec![1, 2]);

        let mut a4: Vec<i32> = vec![2, 1];
        bubble_sort(&mut a4);
        assert_eq!(a4, vec![1, 2]);

        let mut a5: Vec<i32> = vec![1, 2, 4, 3, 6, 5, 8, 7, 9, 10];
        bubble_sort(&mut a5);
        assert_eq!(a5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut a6: Vec<i32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubble_sort(&mut a6);
        assert_eq!(a6, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
