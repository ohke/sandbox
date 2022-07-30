use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![std::i64::MAX; n];

    // 初期値
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();

    // 更新
    for i in 2..n {
        dp[i] = ((h[i] - h[i - 1]).abs() + dp[i - 1]).min((h[i] - h[i - 2]).abs() + dp[i - 2]);
    }

    let result = dp[n - 1];

    println!("{}", result);
}
