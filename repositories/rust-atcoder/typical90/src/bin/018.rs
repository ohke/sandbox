use std::f64::consts::PI;

fn main() {
    proconio::input! {
        t: f64,
    }

    proconio::input! {
        l: f64,
        x: f64,
        y: f64,
    }

    proconio::input! {
        q: usize,
    }

    for _ in 0..q {
        proconio::input! {
            e_i: f64,
        }

        // E869120の座標
        let ex = 0.0;
        let ey = (-1.0) * (2.0 * PI * e_i / t).sin() * l / 2.0;
        let ez = ((2.0 * PI * e_i / t - PI / 2.0).sin() + 1.0) * l / 2.0;

        // 俯角
        let rad = ez.atan2(((ex - x).powf(2.0) + (ey - y).powf(2.0)).sqrt());
        let deg = rad * 180.0 / PI;

        println!("{}", deg);
    }
}
