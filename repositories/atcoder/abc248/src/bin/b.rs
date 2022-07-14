use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
        k: u128,
    }

    let mut count = 0;
    let mut x = a;
    while x < b {
        x *= k;
        count += 1;
    }

    println!("{}", count);
}
