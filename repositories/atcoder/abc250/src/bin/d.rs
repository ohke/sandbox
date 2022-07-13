use num_integer::Roots;

fn main() {
    proconio::input! {
        n: usize,
    }

    let max = (n / 2).cbrt() + 1;

    let mut prime = vec![true; max + 1];
    prime[0] = false;
    prime[1] = false;
    for x in 2..(max + 1) {
        if !prime[x] {
            continue;
        }

        let mut y = x + x;
        while y <= max {
            prime[y] = false;
            y += x;
        }
    }

    let mut ret = 0;
    for q in 0..prime.len() {
        if !prime[q] {
            continue;
        }

        for p in 0..(n / q.pow(3) + 1) {
            if prime[p] {
                ret += 1;
            }
        }
    }

    println!("{}", ret);
}
