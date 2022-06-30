fn main() {
    proconio::input! {
        a: usize,
        b: u32,
        c: usize,
    }

    if a < c.pow(b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
