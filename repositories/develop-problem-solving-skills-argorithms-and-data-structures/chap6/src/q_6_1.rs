fn solve(a: Vec<u64>) -> Vec<u64> {
    let mut a = a;

    let mut original_a = a.clone();

    a.sort();

    for i in 0..original_a.len() {
        let v = original_a[i];

        let mut index = a.len() + 1;
        let mut left = 0;
        let mut right = a.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if a[mid] < v {
                left = mid + 1;
            } else if v < a[mid] {
                right = mid - 1;
            } else {
                index = mid;
                break;
            }
        }

        original_a[i] = index as u64;
    }

    original_a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![12, 43, 7, 15, 9]), vec![2, 4, 0, 3, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![12, 43, 7, 12, 9]), vec![2, 4, 0, 2, 1]);
    }
}
