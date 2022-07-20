use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    }

    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        input! {
            x_i: i64,
            y_i: i64,
        }

        x.push(x_i);
        y.push(y_i);
    }

    input! {
        s: String,
    }

    let mut xy: HashMap<i64, (i64, i64)> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        let v = xy.get(&y[i]);
        let new_v = match v {
            Some((l, r)) => match c {
                'L' => {
                    if !(x[i] < *r) {
                        println!("Yes");
                        return;
                    }

                    (l.clone().max(x[i]), *r)
                }
                'R' => {
                    if !(x[i] > *l) {
                        println!("Yes");
                        return;
                    }

                    (*l, r.clone().min(x[i]))
                }
                _ => panic!(),
            },
            None => match c {
                'L' => (x[i], std::i64::MAX),
                'R' => (-1 as i64, x[i]),
                _ => panic!(),
            },
        };

        xy.insert(y[i], new_v);
    }

    println!("No");
}
