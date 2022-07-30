use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dp = vec![std::i64::MAX; n];

    // 初期化
    dp[0] = 0;

    // 更新
    for i in 0..(n - 1) {
        for j in 1..(k + 1) {
            if i + j >= n {
                break;
            }

            dp[i + j] = dp[i + j].min(dp[i] + (h[i + j] - h[i]).abs());
        }
    }

    let result = dp[n - 1];
    println!("{}", result);
}
