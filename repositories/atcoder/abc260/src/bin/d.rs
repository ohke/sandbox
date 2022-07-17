use std::collections::HashMap;
use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    input! {
        p: [usize; n],
    }

    let mut result = vec![-1; n + 1];

    let mut heads = Vec::new();
    let mut map = HashMap::new();
    for i in 0..n {
        let j = heads.lower_bound(&p[i]);
        if j >= heads.len() {
            heads.push(p[i]);
            map.insert(p[i], vec![p[i]]);
        } else {
            let mut v = map.remove(&heads[j]).unwrap();
            v.push(p[i]);
            map.insert(p[i], v);

            heads[j] = p[i];
        }

        if map.get(&p[i]).unwrap().len() == k {
            heads.remove(j);

            let v = map.remove(&p[i]).unwrap();
            for v_i in v.into_iter() {
                result[v_i] = i as i64 + 1;
            }
        }
    }

    for i in 1..(n + 1) {
        println!("{}", result[i]);
    }
}
