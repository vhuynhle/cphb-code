pub fn binsearch<T>(arr: &[T], x: &T) -> Option<usize>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    let mut l = 0;
    let mut h = arr.len();
    while l < h {
        let mid = l + (h - l) / 2;
        if arr[mid] == *x {
            return Some(mid);
        } else if arr[mid] < *x {
            l = mid + 1;
        } else {
            h = mid;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binsearch;

    #[test]
    fn test_binsearch_method1() {
        assert_eq!(binsearch(&[2], &1), None);
        assert_eq!(binsearch(&[2], &2), Some(0));
        assert_eq!(binsearch(&[1, 2], &1), Some(0));
        assert_eq!(binsearch(&[1, 2], &2), Some(1));
        assert_eq!(binsearch(&[1, 1, 2, 3], &3), Some(3));
        assert_eq!(binsearch(&[1, 1, 2, 3], &2), Some(2));
        let pos = binsearch(&[1, 1, 2, 3], &1);
        assert!(pos == Some(0) || pos == Some(1));
        assert_eq!(binsearch(&[1, 1, 2, 3], &0), None);
        assert_eq!(binsearch(&[1, 1, 2, 3], &4), None);

    }
}
