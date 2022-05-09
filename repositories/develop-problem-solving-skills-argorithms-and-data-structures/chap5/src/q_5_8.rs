fn solve(a: Vec<u64>, m: u64) -> f64 {
    use std::collections::HashMap;

    let m = m as usize;
    let n = a.len();

    let mut means = HashMap::new();
    for i in 1..n + 1 {
        for j in 0..i {
            let mut sum = 0.0;
            for k in j..i {
                sum += a[k] as f64;
            }
            means.insert((j, i), sum / (i - j) as f64);
        }
    }

    println!("means: {:?}", means);

    let mut dp = Vec::with_capacity(n + 1);
    for _ in 0..n + 1 {
        let mut v = Vec::with_capacity(m + 1 as usize);
        for _ in 0..m + 1 {
            v.push(-1.0);
        }
        dp.push(v);
    }

    dp[0][0] = 0.0;

    for i in 0..n + 1 {
        for j in 0..i {
            for k in 1..m + 1 {
                let challenger = dp[j][k - 1] + means.get(&(j, i)).unwrap();
                if dp[i][k] < challenger {
                    dp[i][k] = challenger;
                }
            }
        }
    }

    println!("dp: {:?}", dp);

    let mut ret = 0.0;
    for i in 0..m + 1 {
        if ret < dp[n][i] {
            ret = dp[n][i];
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![9, 1, 2, 3, 9], 3), 20.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![14, 4, 9, 7], 1), 8.5);
    }
}
