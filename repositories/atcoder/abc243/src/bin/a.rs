use proconio::input;

fn main() {
    input! {
        mut v: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    v %= a + b + c;
    let result = if v < a {
        "F"
    } else if v < (a + b) {
        "M"
    } else {
        "T"
    };

    println!("{}", result);
}
