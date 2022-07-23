use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; n],
    }
    x.insert(0, 0);

    let mut b = vec![0; n+1];
    for _ in 0..m {
        input! {
            c: usize,
            y: i64,
        }
        b[c] = y;
    }

    let mut dp = vec![vec![-1_i64; n+1]; n+1];

    // 初期値
    dp[0][0] = 0;

    // 更新
    for i in 1..(n+1) {
        // i回目が裏
        dp[i][0] = dp[i-1].iter().max().unwrap().clone();

        // i回目が表
        for j in 1..(i+1) {
            dp[i][j] = dp[i-1][j-1] + x[i] + b[j];
        }
    }

    let result = dp[n].iter().max().unwrap().clone();
    println!("{}", result);
}
