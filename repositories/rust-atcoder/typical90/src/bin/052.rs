fn main() {
    proconio::input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut ret = 1;
    for i in 0..n {
        let p: usize = a[i].iter().sum();
        ret *= p;
        ret %= 1000000007;
    }

    println!("{}", ret);
}
