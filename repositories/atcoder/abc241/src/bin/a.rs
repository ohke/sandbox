use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }

    let mut result = 0;
    for _ in 0..3 {
        result = a[result];
    }

    println!("{}", result);
}
