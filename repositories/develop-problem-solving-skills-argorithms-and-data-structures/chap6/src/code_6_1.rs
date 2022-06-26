fn binary_search(a: Vec<u8>, key: u8) -> i32 {
    let mut left = 0;
    let mut right = a.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if a[mid] == key {
            return mid as i32;
        } else if a[mid] > key {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    -1 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(binary_search(vec![3, 5, 8, 10, 14, 17, 21, 39], 10), 3,)
    }

    #[test]
    fn test_2() {
        assert_eq!(binary_search(vec![3, 5, 8, 10, 14, 17, 21, 39], 9), -1,)
    }
}
