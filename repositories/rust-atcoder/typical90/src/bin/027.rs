use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: usize,
    }

    let mut names = HashSet::new();
    for i in 1..(n + 1) {
        proconio::input! {
            s_i: String,
        }

        if names.insert(s_i) {
            println!("{}", i);
        }
    }
}
