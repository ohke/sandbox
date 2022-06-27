fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    proconio::input! {
        a: [[u64; w]; h],
    }

    let mut row = vec![0; h];
    let mut column = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row[i] += a[i][j];
            column[j] += a[i][j];
        }
    }

    let mut m = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            m[i][j] = row[i] + column[j] - a[i][j]
        }
    }

    for i in 0..h {
        let v: Vec<String> = m[i].iter().map(|m_i| m_i.to_string()).collect();
        println!("{}", v.join(" "));
    }
}
