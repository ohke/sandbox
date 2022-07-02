fn main() {
    proconio::input! {
        a: u64,
        b: u64,
    }

    let gcd;
    let mut x = a;
    let mut y = b;
    loop {
        if x < y {
            y = y % x;
            if y == 0 {
                gcd = x;
                break;
            }
        } else {
            x = x % y;
            if x == 0 {
                gcd = y;
                break;
            }
        }
    }

    if b / gcd <= 1000000000000000000 / a {
        println!("{}", a * b / gcd);
    } else {
        println!("Large");
    }
}
