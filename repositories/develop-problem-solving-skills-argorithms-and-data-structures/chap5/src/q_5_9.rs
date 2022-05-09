fn solve(a: Vec<u64>) -> u64 {
    const MAX_COST: u64 = std::u64::MAX;
    let n = a.len();

    let mut s = Vec::with_capacity(n + 1);
    s.push(0);
    for i in 0..n {
        s.push(s[i] + a[i]);
    }

    let mut dp = Vec::with_capacity(n + 1);
    for _ in 0..n + 1 {
        let mut row = Vec::with_capacity(n + 1);
        for _ in 0..n + 1 {
            row.push(MAX_COST);
        }
        dp.push(row);
    }
    for i in 0..n {
        dp[i][i + 1] = 0;
    }

    for bet in 2..n + 1 {
        for i in 0..(n + 1 - bet) {
            let j = i + bet;
            for k in i + 1..j {
                let challenger = dp[i][k] + dp[k][j] + s[j] - s[i];
                if challenger < dp[i][j] {
                    dp[i][j] = challenger;
                }
            }
        }
    }

    println!("dp: {:?}", dp);

    dp[0][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![10, 20, 30, 40]), 190);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![10, 10, 10, 10, 10]), 120);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![1000000000, 1000000000, 1000000000]), 5000000000);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(vec![7, 6, 8, 6, 1, 1]), 68);
    }
}
