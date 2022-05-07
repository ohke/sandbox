fn solve(a: Vec<u64>, w: u64) -> bool {
    let mut dp = [[false; 10001]; 100];
    dp[0][0] = true;

    let w = w as usize;

    for i in 0..a.len() {
        for j in 0..w + 1 {
            dp[i + 1][j] |= dp[i][j];
            if a[i] as usize <= j {
                dp[i + 1][j] |= dp[i + 1][j - a[i] as usize];
            }
        }
    }

    dp[a.len()][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![7, 5, 3], 10), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![9, 7], 10), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![8, 3, 4], 10), true);
    }
}
