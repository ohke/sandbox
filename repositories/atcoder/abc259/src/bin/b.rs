use std::f64::consts::PI;

fn main() {
    proconio::input! {
        a: f64,
        b: f64,
        d: f64,
    }

    let r = (a.powf(2.0) + b.powf(2.0)).sqrt();
    let rad = b.atan2(a) + (d / 180.0 * PI);
    println!("{} {}", r * rad.cos(), r * rad.sin())
}
