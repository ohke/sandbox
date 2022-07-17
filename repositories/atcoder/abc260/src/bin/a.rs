use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let c: Vec<char> = s.chars().collect();
    if c[0] != c[1] && c[0] != c[2] {
        println!("{}", c[0]);
    } else if c[1] != c[0] && c[1] != c[2] {
        println!("{}", c[1]);
    } else if c[2] != c[0] && c[2] != c[1] {
        println!("{}", c[2]);
    } else {
        println!("-1");
    }
}
