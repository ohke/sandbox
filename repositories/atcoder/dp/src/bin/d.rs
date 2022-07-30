use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
    }

    let mut dp = vec![vec![0_usize; 100001]; n + 1];
    for i in 1..n + 1 {
        input! {
            w_i: usize,
            v_i: usize,
        }

        for j in 1..w + 1 {
            dp[i][j] = if j >= w_i {
                dp[i - 1][j].max(dp[i - 1][j - w_i] + v_i)
            } else {
                dp[i - 1][j]
            }
        }
    }

    let result = dp[n][w];
    println!("{}", result);
}
