use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
    }

    let v = 1000 * 100;
    let mut dp = vec![vec![w+1; v+1]; n+1];

    dp[0][0] = 0;

    for i in 1..n+1 {
        input! {
            w_i: usize,
            v_i: usize,
        }

        for j in 0..v+1 {
            dp[i][j] = if j >= v_i {
                dp[i-1][j].min(dp[i-1][j-v_i] + w_i)
            } else {
                dp[i-1][j]
            }
        }
    }

    for i in (1..v).rev() {
        if dp[n][i] <= w {
            println!("{}", i);
            return;
        }
    }
}
