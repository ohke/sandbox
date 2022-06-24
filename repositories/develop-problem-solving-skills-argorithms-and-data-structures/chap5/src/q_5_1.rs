fn solve(n: usize, a: Vec<u64>, b: Vec<u64>, c: Vec<u64>) -> u64 {
    let mut dp = vec![vec![0; 3]; 2];

    dp[0][0] = a[0];
    dp[0][1] = b[0];
    dp[0][2] = c[0];

    for i in 1..n {
        let i_1 = i % 2;
        let i_0 = (i - 1) % 2;

        for j in 0..3 {
            dp[i_1][j] = match j {
                0 => {
                    if dp[i_0][1] < dp[i_0][2] {
                        dp[i_0][2] + a[i]
                    } else {
                        dp[i_0][1] + a[i]
                    }
                }
                1 => {
                    if dp[i_0][0] < dp[i_0][2] {
                        dp[i_0][2] + b[i]
                    } else {
                        dp[i_0][0] + b[i]
                    }
                }
                2 => {
                    if dp[i_0][0] < dp[i_0][1] {
                        dp[i_0][1] + c[i]
                    } else {
                        dp[i_0][0] + c[i]
                    }
                }
                _ => panic!(),
            }
        }
    }

    *dp[(n - 1) % 2].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(3, vec![10, 20, 30], vec![40, 50, 60], vec![70, 80, 90]),
            210
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(1, vec![100], vec![10], vec![1]), 100);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                7,
                vec![6, 8, 2, 7, 4, 2, 7],
                vec![7, 8, 5, 8, 6, 3, 5],
                vec![8, 3, 2, 6, 8, 4, 1]
            ),
            46
        );
    }
}
