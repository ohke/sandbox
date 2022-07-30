use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut c: Vec<char> = s.chars().collect();
    c.sort();

    let result: String = c.iter().collect();
    println!("{}", result);
}
