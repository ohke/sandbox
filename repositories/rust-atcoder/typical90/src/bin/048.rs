use std::collections::BinaryHeap;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }

    let mut heap = BinaryHeap::new();
    for _ in 0..n {
        proconio::input! {
            a_i: usize,
            b_i: usize,
        }

        heap.push((b_i, a_i - b_i));
    }

    let mut ret = 0;
    for _ in 0..k {
        let (b, a) = heap.pop().unwrap();

        ret += b;
        if a != 0 {
            heap.push((a, 0));
        }
    }

    println!("{}", ret);
}
