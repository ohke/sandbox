fn solve(a: Vec<u64>, m: Vec<u64>, w: u64) -> bool {
    const MAX_M: usize = 1001;

    let w = w as usize;

    let mut dp = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        dp.push(Vec::with_capacity(w + 1));
        for _ in 0..w + 1 {
            dp[i].push(MAX_M);
        }
    }
    dp[0][0] = 0;

    for i in 0..a.len() {
        for j in 0..w + 1 {
            if 0 < i && dp[i - 1][j] < MAX_M {
                dp[i][j] = 0;
            }
            if a[i] as usize <= j && dp[i][j - a[i] as usize] < m[i] as usize {
                dp[i][j] = dp[i][j - a[i] as usize] + 1;
            }
        }
    }

    dp[a.len() - 1][w] < MAX_M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![7, 5, 3], vec![1, 1, 1], 10), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![9, 7], vec![1, 1], 10), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![8, 3, 4], vec![2, 1, 2], 10), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(vec![8, 3, 4], vec![2, 2, 2], 10), true);
    }
}
