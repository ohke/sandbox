fn main() {
    proconio::input! {
        n: usize,
        l: usize,
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..(n + 1) {
        if i < l {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = (dp[i - 1] + dp[i - l]) % 1000000007;
        }
    }

    println!("{}", dp[n]);
}
