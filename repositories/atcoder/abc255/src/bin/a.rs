fn main() {
    proconio::input! {
        r: usize,
        c: usize,
    }

    proconio::input! {
        a: [[u8; 2]; 2],
    }

    println!("{}", a[r - 1][c - 1]);
}
