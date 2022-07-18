fn main() {
    proconio::input! {
        n: u128,
    }

    let mut cb = (n as f64).cbrt().ceil() as u128;
    loop {
        let d = cb * cb * cb - n;

        let mut a = 1;
        let mut b = cb as u128 - 1;

        while a <= b {
            if d == 2 * (a * a * b + a * b * b) {
                println!("{}", cb * cb * cb - 2 * (a * a * b + a * b * b));
                return;
            }

            a += 1;
            b -= 1;
        }

        cb += 1;
    }
}
