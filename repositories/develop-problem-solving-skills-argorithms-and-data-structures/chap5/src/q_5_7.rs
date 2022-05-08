fn solve(s: String, t: String) -> u64 {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut dp = Vec::with_capacity(s.len() + 1);
    for i in 0..s.len() + 1 {
        dp.push(Vec::with_capacity(t.len() + 1));
        for _ in 0..t.len() + 1 {
            dp[i].push(0);
        }
    }

    for i in 1..s.len() + 1 {
        let s_i = i - 1;
        for j in 1..t.len() + 1 {
            let t_j = j - 1;

            if s[s_i] == t[t_j] {
                let challenger = dp[i - 1][j - 1] + 1;
                dp[i][j] = if dp[i - 1][j] < dp[i][j - 1] {
                    if challenger < dp[i][j - 1] {
                        dp[i][j - 1]
                    } else {
                        challenger
                    }
                } else {
                    if challenger < dp[i - 1][j] {
                        dp[i - 1][j]
                    } else {
                        challenger
                    }
                }
            } else {
                dp[i][j] = if dp[i - 1][j] < dp[i][j - 1] {
                    dp[i][j - 1]
                } else {
                    dp[i - 1][j]
                }
            }
        }
    }

    dp[s.len()][t.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("abcde".to_string(), "abcd".to_string()), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("bcde".to_string(), "abcde".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("abcde".to_string(), "fghij".to_string()), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("abcde".to_string(), "fbhdj".to_string()), 2);
    }
}
