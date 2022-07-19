use std::io::{stdout, Write};

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![false; 2 * n + 2];
    let mut count = 0;
    loop {
        for i in 1..2 * n + 2 {
            if !a[i] {
                a[i] = true;
                println!("{}", i);
                stdout().flush().unwrap();
                break;
            }
        }
        count += 1;

        if count == 2 * n + 1 {
            println!("0");
            stdout().flush().unwrap();
            break;
        }

        input! {
            x: usize,
        }
        a[x] = true;
        count += 1;
    }
}
