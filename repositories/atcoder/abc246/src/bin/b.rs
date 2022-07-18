use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let d = (a.powf(2.0) + b.powf(2.0)).sqrt();

    println!("{} {}", a / d, b / d);
}
