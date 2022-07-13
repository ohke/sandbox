use im_rc::HashMap;
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (1..n + 1).collect();

    let mut hash = HashMap::new();
    for (i, a_i) in a.iter().enumerate() {
        hash.insert(a_i.clone(), i);
    }

    for _ in 0..q {
        proconio::input! {
            x: usize,
        }

        let i = hash.get(&x).unwrap().clone();
        let j = if i == n - 1 { i - 1 } else { i + 1 };

        a.swap(i, j);

        hash.insert(a[i], i);
        hash.insert(a[j], j);
    }

    let s = a.iter().map(|a_i| a_i.to_string()).join(" ");
    println!("{}", &s);
}
