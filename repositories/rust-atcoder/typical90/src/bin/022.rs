fn gcd(mut x: usize, mut y: usize) -> usize {
    loop {
        if x >= y {
            x = x % y;
            if x == 0 {
                return y;
            }
        } else {
            y = y % x;
            if y == 0 {
                return x;
            }
        }
    }
}

fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let r = gcd(a, gcd(b, c));

    let ret = (a/r - 1) + (b/r - 1) + (c/r - 1);

    println!("{}", ret);
}
