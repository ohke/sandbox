fn upper_bound(a: &Vec<u64>, v: u64) -> u64 {
    if v < a[0] {
        return 0;
    } else if a[a.len() - 1] <= v {
        return a.len() as u64;
    }

    let mut left = 0;
    let mut right = a.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if a[mid] <= v {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left as u64
}

fn solve(a: Vec<u64>, b: Vec<u64>, k: u64) -> u64 {
    let mut b = b;
    b.sort();

    let mut left = 0;
    let mut right = u64::MAX;
    while left < right {
        let mid = left + (right - left) / 2;

        let mut count = 0;
        for i in 0..a.len() {
            count += upper_bound(&b, mid / a[i]);
        }

        if count >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![2, 3], vec![3, 5], 3), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![1, 2, 1], vec![2, 1, 2], 7), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                vec![701687787, 500872619, 516391519, 599949380],
                vec![458299111, 633119409, 377269575, 717229869],
                8
            ),
            317112176525562171
        );
    }

    #[test]
    fn test_upper_bound_1() {
        assert_eq!(upper_bound(&vec![1, 1, 2, 3, 3, 3, 4], 3), 6);
    }
}
