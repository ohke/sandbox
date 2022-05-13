fn solve(a: Vec<u64>, m: u64) -> u64 {
    let mut ret = 0;

    let mut left = 0;
    let mut right = usize::MAX;
    while left <= right {
        let mid = (left + (right - left) / 2) as u64;

        let mut prev = a[0];
        let mut count = 1;
        for i in 1..a.len() {
            if mid <= (a[i] - prev) {
                prev = a[i];
                count += 1;
                if count == m {
                    break;
                }
            }
        }

        if count == m {
            ret = mid;
            left = (mid + 1) as usize;
        } else {
            right = (mid - 1) as usize;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![1, 2, 4, 8, 9], 3), 3);
    }
}
