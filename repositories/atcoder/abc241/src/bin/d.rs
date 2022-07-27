use std::collections::BTreeMap;
use std::ops::Bound::Included;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for _ in 0..q {
        input! {
            q_i: usize,
        }

        match q_i {
            1 => {
                input! {
                    x: usize,
                }

                match map.get(&x) {
                    Some(c) => map.insert(x, c + 1),
                    None => map.insert(x, 1_usize),
                };
            }
            2 => {
                input! {
                    x: usize,
                    k: usize,
                }

                let mut count = 0;
                for (&key, &c) in map.range((Included(&0), (Included(&x)))).rev() {
                    count += c;
                    if count >= k {
                        println!("{}", &key);
                        break;
                    }
                }

                if count < k {
                    println!("-1");
                }
            }
            3 => {
                input! {
                    x: usize,
                    k: usize,
                }

                let mut count = 0;
                for (&key, &c) in map.range((Included(&x), (Included(&std::usize::MAX)))) {
                    count += c;
                    if count >= k {
                        println!("{}", &key);
                        break;
                    }
                }

                if count < k {
                    println!("-1");
                }
            }
            _ => panic!(),
        }
    }
}
