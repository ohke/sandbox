fn main() {
    proconio::input! {
        n: usize,
    }

    proconio::input! {
        mut a: [i64; n],
    }

    proconio::input! {
        mut b: [i64; n],
    }

    a.sort();
    b.sort();

    let mut ret = 0;
    for i in 0..n {
        ret += (a[i] - b[i]).abs();
    }

    println!("{}", ret);
}
