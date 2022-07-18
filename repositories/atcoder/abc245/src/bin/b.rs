use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = vec![false; 2001];

    for i in 0..n {
        result[a[i]] = true;
    }

    for i in 0..2001 {
        if !result[i] {
            println!("{}", i);
            return;
        }
    }
}
