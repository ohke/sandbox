fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }

    if x <= m {
        println!("{}", t);
    } else {
        let ret = t - (x - m) * d;
        println!("{}", ret);
    }
}
