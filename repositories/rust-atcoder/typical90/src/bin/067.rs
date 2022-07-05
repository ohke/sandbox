fn main() {
    proconio::input! {
        mut n: String,
        k: usize,
    }

    for _ in 0..k {
        // 8進数文字列 -> u64
        let mut x = u64::from_str_radix(&n, 8).unwrap();

        // u64 -> 9進数文字列
        let mut base = 1;
        let mut y = 0;
        loop {
            y += (x % 9) * base;

            x /= 9;
            if x == 0 {
                break;
            }

            base *= 10;
        }

        let y = y.to_string();

        // 8 -> 5
        n = y.replace("8", "5");
    }

    println!("{}", n);
}
