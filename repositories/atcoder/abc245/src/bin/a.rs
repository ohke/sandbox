use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let result = if a < c {
        "Takahashi"
    } else if c < a {
        "Aoki"
    } else if d < b {
        "Aoki"
    } else {
        "Takahashi"
    };

    println!("{}", result);
}
