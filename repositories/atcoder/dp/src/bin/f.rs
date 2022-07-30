use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s = s.chars().collect_vec();
    let t = t.chars().collect_vec();

    let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];

    for i in 1..(t.len() + 1) {
        for j in 1..(s.len() + 1) {
            dp[i][j] = if s[j - 1] == t[i - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }

    let mut result: Vec<char> = Vec::new();
    let mut i = t.len();
    let mut j = s.len();
    while dp[i][j] != 0 {
        if dp[i][j] == dp[i - 1][j - 1] {
            j -= 1;
        } else {
            result.insert(0, s[j - 1]);
            i -= 1;
            j -= 1;
        }
    }

    let result: String = result.into_iter().collect();
    if result.len() > 0 {
        println!("{}", &result);
    }
}
