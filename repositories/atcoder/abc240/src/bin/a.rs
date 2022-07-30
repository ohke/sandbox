use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let d = (a - b).abs();
    if d == 1 || d == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
