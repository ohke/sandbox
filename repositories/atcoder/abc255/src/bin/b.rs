fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }

    let mut a = vec![false; n];
    for _ in 0..k {
        proconio::input! {
            i: usize,
        }
        a[i - 1] = true;
    }

    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    for _ in 0..n {
        proconio::input! {
            x_i: f64,
            y_i: f64,
        }

        x.push(x_i);
        y.push(y_i);
    }

    let mut r: f64 = 0.0;
    for i in 0..n {
        if a[i] {
            continue;
        }

        let mut r_i = std::f64::MAX;
        for j in 0..n {
            if !a[j] {
                continue;
            }
            r_i = r_i.min(((x[i] - x[j]).powf(2.0) + (y[i] - y[j]).powf(2.0)).sqrt());
        }

        r = r.max(r_i);
    }

    println!("{}", r);
}
