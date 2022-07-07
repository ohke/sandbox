fn main() {
    proconio::input! {
        mut n: usize,
    }

    let mut n_primes = 0;
    for i in 2..((n as f64).sqrt().ceil() as usize + 1) {
        loop {
            if n % i == 0 {
                n_primes += 1;
                n /= i;
            } else {
                break;
            }
        }
    }

    let ret = if n_primes == 0 {
        0
    } else {
        f64::log2(n_primes as f64).ceil() as usize
    };

    println!("{}", ret);
}
