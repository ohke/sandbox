use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        input! {
            s: String,
        }

        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                a[i][j] = 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            // 右
            if (n - j) >= 6 {
                let mut sum = 0;
                for d_j in 0..6 {
                    sum += a[i][j + d_j];
                }
                if sum >= 4 {
                    println!("Yes");
                    return;
                }
            }
            // 下
            if (n - i) >= 6 {
                let mut sum = 0;
                for d_i in 0..6 {
                    sum += a[i + d_i][j];
                }
                if sum >= 4 {
                    println!("Yes");
                    return;
                }
            }
            // 右下
            if (n - i) >= 6 && (n - j) >= 6 {
                let mut sum = 0;
                for d in 0..6 {
                    sum += a[i + d][j + d];
                }
                if sum >= 4 {
                    println!("Yes");
                    return;
                }
            }
            // 左下
            if (n - i) >= 6 && j >= 5 {
                let mut sum = 0;
                for d in 0..6 {
                    sum += a[i + d][j - d];
                }
                if sum >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
