use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();
    for a_i in a.iter().cloned() {
        set.insert(a_i);
    }
    
    println!("{}", set.len());
}
