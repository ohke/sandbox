fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    proconio::input! {
        mut a: [[i64; w]; h],
    }

    proconio::input! {
        b: [[i64; w]; h],
    }

    let mut count = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let d = b[i][j] - a[i][j];
            a[i][j] += d;
            a[i][j + 1] += d;
            a[i + 1][j] += d;
            a[i + 1][j + 1] += d;

            count += d.abs();
        }
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes\n{}", count);
}
