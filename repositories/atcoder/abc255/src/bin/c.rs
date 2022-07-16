fn main() {
    proconio::input! {
        x: i128,
        mut a: i128,
        mut d: i128,
        n: i128,
    }

    if d < 0 {
        a = d * (n - 1) + a;
        d = -d;
    }

    let z = a + d * (n - 1);

    if x <= a {
        println!("{}", (x - a).abs());
    } else if x >= z {
        println!("{}", (x - z).abs());
    } else {
        let l = a + ((x - a) / d) * d;
        let r = a + ((x - a) / d + 1) * d;
        println!("{}", (x - l).abs().min((x - r).abs()));
    }
}
