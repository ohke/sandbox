use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    input! {
        a: [usize; n],
        b: [usize; n],
    }

    let mut passed = vec![false; n];

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((a[i], Reverse(i)));
    }
    for _ in 0..x {
        let (_, Reverse(i)) = heap.pop().unwrap();
        passed[i] = true;
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((b[i], Reverse(i)));
    }
    let mut count = 0;
    while count < y {
        let (_, Reverse(i)) = heap.pop().unwrap();
        if passed[i] {
            continue;
        }
        passed[i] = true;
        count += 1;
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((a[i] + b[i], Reverse(i)));
    }
    let mut count = 0;
    while count < z {
        let (_, Reverse(i)) = heap.pop().unwrap();
        if passed[i] {
            continue;
        }
        passed[i] = true;
        count += 1;
    }

    for i in 0..n {
        if passed[i] {
            println!("{}", i + 1);
        }
    }
}
