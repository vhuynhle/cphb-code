pub fn binsearch<T>(arr: &[T], x: &T) -> Option<usize>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    let mut index = 0;
    let mut jump = arr.len() / 2;
    while jump > 0 {
        while index + jump < arr.len() && arr[index + jump] <= *x {
            index += jump;
        }
        jump /= 2;
    }
    if arr[index] == *x {
        return Some(index);
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
