fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    if h == 1 || w == 1 {
        println!("{}", h * w);
        return;
    }

    let mut m = vec![vec![0; w]; h];
    let mut ret = 0;

    for i in 0..h {
        for j in 0..w {
            if i > 0 && m[i - 1][j] == 1 {
                continue;
            }
            if j > 0 && m[i][j - 1] == 1 {
                continue;
            }
            if i > 0 && j > 0 && m[i - 1][j - 1] == 1 {
                continue;
            }

            m[i][j] = 1;
            ret += 1;
        }
    }

    println!("{}", ret);
}
