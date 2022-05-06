fn solve(a: Vec<u64>, w: u64) -> u64 {
    let mut dp = Vec::with_capacity(101);
    dp.push(vec![0; 13]);

    let w = w as usize;

    for i in 1..a.len() + 1 {
        dp.push(dp[i - 1].clone());

        for j in 0..w {
            let j = w - j;

            if (a[i - 1] as usize) < j && 0 < dp[i][j - (a[i - 1] as usize)] {
                dp[i][j] += 1;
            }
        }

        dp[i][a[i - 1] as usize] += 1;
    }

    println!("{:?}", dp);

    dp[a.len()][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![7, 5, 3, 1, 8], 12), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![4, 1, 1, 1], 5), 3);
    }
}
