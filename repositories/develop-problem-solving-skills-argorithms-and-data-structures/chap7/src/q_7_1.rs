fn solve(a: Vec<i64>, b: Vec<i64>) -> u64 {
    let mut ret = 0;

    let mut b = b;
    b.sort();

    for i in 0..a.len() {
        // upper bound
        let mut left = 0;
        let mut right = b.len() - 1;

        if b[right] <= a[i] {
            continue;
        } else if b[left] > a[i] {
            ret += b.len() as u64;
        } else {
            while left < right {
                let mid = left + (right - left) / 2;
                if b[mid] <= a[i] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            ret += left as u64;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![1, 2], vec![2, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![1, 2], vec![3, 4]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![3, 4], vec![1, 2]), 0);
    }
}
