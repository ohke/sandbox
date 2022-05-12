fn lower_bound(a: &Vec<u64>, v: u64) -> Option<usize> {
    // v < a[i] を満たす最小の i を返す
    let mut left = 0;
    let mut right = a.len() - 1;

    if a[right] < v {
        return None;
    } else if v <= a[left] {
        return Some(0);
    }

    while left <= right {
        let mid = left + (right - left) / 2;
        if a[mid] < v {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    Some(left)
}

fn solve(a: Vec<u64>, b: Vec<u64>, c: Vec<u64>) -> u64 {
    let mut ret = 0;

    let n = a.len();
    let mut a = a;
    let mut b = b;
    let mut c = c;

    a.sort();
    b.sort();
    c.sort();

    for j in 0..n {
        let n_a = match lower_bound(&a, b[j]) {
            Some(i) => i,
            None => n,
        };

        let n_c = match lower_bound(&c, b[j] + 1) {
            Some(k) => n - k,
            None => 0,
        };

        ret += n_a * n_c;
    }

    ret as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![1, 5], vec![2, 4], vec![3, 6]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]), 27);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                vec![3, 14, 159, 2, 6, 53],
                vec![58, 9, 79, 323, 84, 6],
                vec![2643, 383, 2, 79, 50, 288]
            ),
            87
        );
    }

    #[test]
    fn test_lower_bound_1() {
        assert_eq!(lower_bound(&vec![1, 1, 2, 3, 3, 4], 3), Some(3));
    }

    #[test]
    fn test_lower_bound_2() {
        assert_eq!(lower_bound(&vec![1, 1, 2, 3, 3, 4], 1), Some(0));
    }

    #[test]
    fn test_lower_bound_3() {
        assert_eq!(lower_bound(&vec![1, 1, 2, 3, 3, 4], 4), Some(5));
    }

    #[test]
    fn test_lower_bound_4() {
        assert_eq!(lower_bound(&vec![1, 1, 2, 3, 3, 4], 5), None);
    }
}
