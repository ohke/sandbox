use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let m: usize = 998244353;
    
    let mut dp = vec![vec![0; 11]; n+1];

    // 初期値
    for i in 1..10 {
        dp[1][i] = 1;
    }

    // 更新
    for i in 2..(n+1) {
        for j in 1..10 {
            for k in j-1..j+2 {
                dp[i][k] = (dp[i][k] + dp[i-1][j]) % m;
            }
        }
    }

    // 結果はn桁目の総和
    let mut result = 0;
    for i in 1..10 {
        result = (result + dp[n][i]) % m;
    }

    println!("{}", result);
}
