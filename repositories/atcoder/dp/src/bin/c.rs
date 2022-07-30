use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 1..n + 1 {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }

        dp[i][0] = a + dp[i - 1][1].max(dp[i - 1][2]);
        dp[i][1] = b + dp[i - 1][0].max(dp[i - 1][2]);
        dp[i][2] = c + dp[i - 1][0].max(dp[i - 1][1]);
    }

    let result = dp[n].iter().max().unwrap();
    println!("{}", result);
}
