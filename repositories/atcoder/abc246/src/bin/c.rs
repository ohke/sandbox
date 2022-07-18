use std::collections::BinaryHeap;

fn main() {
    proconio::input! {
        n: usize,
        mut k: i64,
        x: i64,
    }

    proconio::input! {
        a: [i64; n],
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(a[i]);
    }

    let mut result = 0;
    while let Some(a_i) = heap.pop() {
        if k > 0 {
            let c = a_i as f64 / x as f64;
            let k_i = if c < 1.0 { 1 } else { a_i / x };

            let k_i = k.min(k_i);

            let a_i = a_i - k_i * x;
            if a_i > 0 {
                heap.push(a_i);
            }

            k -= k_i;
        } else {
            result += a_i;
        }
    }

    println!("{}", result);
}
