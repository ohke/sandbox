fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
    }

    let mut b = vec![0; n - 1];
    let mut ret = 0;
    for i in 1..n {
        b[i - 1] = a[i] - a[i - 1];
        ret += b[i - 1].abs();
    }

    for _ in 0..q {
        proconio::input! {
            mut l: usize,
            mut r: usize,
            v: i64,
        }

        if l > 1 {
            ret -= b[l - 2].abs();
            b[l - 2] = b[l - 2] + v;
            ret += b[l - 2].abs();
        }

        if r < n {
            ret -= b[r - 1].abs();
            b[r - 1] = b[r - 1] - v;
            ret += b[r - 1].abs();
        }

        println!("{}", ret);
    }
}
