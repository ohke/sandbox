fn solve(n: u64) -> u64 {
    let mut dp = vec![0; 3];

    for i in 0..n + 1 {
        let v = if i == 0 || i == 1 {
            0u64
        } else if i == 2 {
            1u64
        } else {
            dp[0] + dp[1] + dp[2]
        };

        dp[0] = dp[1];
        dp[1] = dp[2];
        dp[2] = v;
    }

    dp[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(8), 24);
    }
}
