use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut sum = 0;
    for c in s.chars() {
        sum += c as usize - '0' as usize;
    }

    println!("{}", 45 - sum);
}
