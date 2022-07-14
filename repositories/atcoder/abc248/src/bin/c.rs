fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let MOD: usize = 998244353;

    let mut dp = vec![vec![0_usize; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k {
            for l in 1..(m + 1) {
                if j + l <= k {
                    dp[i + 1][j + l] = (dp[i + 1][j + l] + dp[i][j]) % MOD;
                }
            }
        }
    }

    println!("{:?}", dp);

    let mut ret = 0;
    for i in 1..(k + 1) {
        ret = (ret + dp[n][i]) % MOD;
    }

    println!("{}", ret);
}
