use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let c: Vec<char> = s.chars().collect();

    println!("{}", c[n - 1]);
}
