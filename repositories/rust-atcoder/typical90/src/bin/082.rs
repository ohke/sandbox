fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut p = 1_usize;
    let mut q = a;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            p *= q;
            p %= m;
        }

        q *= q;
        q %= m;
    }

    p
}

fn div(a: usize, b: usize, m: usize) -> usize {
    (a * modpow(b, m - 2, m)) % m
}

fn f(x: usize, m: usize) -> usize {
    let v1 = x % m;
    let v2 = (x + 1) % m;
    let v = v1 * v2 % m;

    div(v, 2, m)
}

fn main() {
    proconio::input! {
        l: usize,
        r: usize,
    }

    let m = 1000000007;

    let mut ret = 0;
    for i in 1..19 {
        let left = l.max(10_usize.pow((i - 1) as u32));
        let right = r.min(10_usize.pow(i as u32) - 1);
        if left > right {
            continue;
        }

        let v = (m + f(right, m) - f(left - 1, m)) % m;
        println!("{} {} {}", left, right, v);
        ret += i * v;
        ret %= m;
    }

    println!("{}", ret);
}
