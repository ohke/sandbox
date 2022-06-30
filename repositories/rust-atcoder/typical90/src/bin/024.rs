fn main() {
    proconio::input! {
        n: usize,
        k: i64,
    }

    proconio::input! {
        a: [i64; n],
        b: [i64; n],
    }

    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] - b[i]).abs();
    }

    if k >= diff && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
