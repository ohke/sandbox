use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut r = vec![0; n];
    r[n - 1] = 1;

    let mut b = vec![0; n];

    for i in (0..(n - 1)).rev() {
        r[i] += r[i + 1];
        b[i + 1] += r[i + 1] * x;

        r[i] += b[i + 1];
        b[i] += b[i + 1] * y;
    }

    println!("{}", b[0]);
}
