fn solve(a: Vec<u64>, k: usize, w: u64) -> bool {
    const MAX_A_LEN: u64 = 101;
    const MAX_W: usize = 11;

    let mut dp = Vec::with_capacity(MAX_A_LEN as usize);
    dp.push(vec![MAX_A_LEN; MAX_W]);
    dp[0][0] = 0;

    let w = w as usize;

    for i in 1..a.len() + 1 {
        dp.push(vec![MAX_A_LEN; MAX_W]);

        for j in 0..w + 1 {
            dp[i][j] = dp[i - 1][j];
            if (a[i - 1] as usize) <= j {
                let challenger = dp[i][j - a[i - 1] as usize] + 1;
                if challenger < dp[i][j] {
                    dp[i][j] = challenger;
                }
            }
        }
    }

    dp[a.len()][w] <= (k as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![7, 5, 3], 2, 10), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![9, 7], 2, 10), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![7, 5, 3], 1, 10), false);
    }
}
