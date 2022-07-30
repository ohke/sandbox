use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut dp = vec![vec![false; x+1]; n+1];
    
    dp[0][0] = true;

    for i in 1..(n+1) {
        input! {
            a: usize,
            b: usize,
        }

        for j in 1..(x+1) {
            if j >= a {
                if dp[i-1][j-a] {
                    dp[i][j] = true;
                }
            }
            if j >= b {
                if dp[i-1][j-b] {
                    dp[i][j] = true;
                }
            }
        }
    }

    let result = if dp[n][x] {
        "Yes"
    } else {
        "No"
    };

    println!("{}", result);
}
