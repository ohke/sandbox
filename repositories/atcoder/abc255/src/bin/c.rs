use core::panic;

fn main() {
    proconio::input! {
        x: i128,
        a: i128,
        d: i128,
        n: i128,
    }

    let ret = if (d == 0) || (x <= a && d >= 0) || (x >= a && d <= 0) {
        (x - a).abs()
    } else if ((x >= a + d * (n - 1)) && d > 0) || ((x <= a + d * (n - 1)) && d < 0) {
        (x - (a + d * (n - 1))).abs()
    } else if d > 0 {
        let l = (x - a) / d;
        let d_l = x - (a + d * l);

        let r = (x - a) / d + 1;
        let d_r = (a + d * r) - x;

        d_l.min(d_r)
    } else if d < 0 {
        let l = ((a - x) / d).abs();
        let d_l = (a + d * l) - x;
        println!("l: {}, d_l: {}", l, d_l);

        let r = ((a - x) / (d - 1)).abs();
        let d_r = x - (a + d * r);
        println!("r: {}, d_r: {}", r, d_r);

        d_l.min(d_r)
    } else {
        panic!();
    };

    println!("{}", ret);
}
