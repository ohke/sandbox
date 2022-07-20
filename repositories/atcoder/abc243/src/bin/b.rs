use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let a_set: HashSet<_> = a.iter().cloned().collect();
    let b_set: HashSet<_> = b.iter().cloned().collect();
    let u_set: HashSet<_> = a_set.union(&b_set).collect();

    let mut count = 0;
    for i in 0..n {
        if a[i] == b[i] {
            count += 1;
        }
    }

    println!(
        "{}\n{}",
        count,
        a_set.len() + b_set.len() - u_set.len() - count
    );
}
