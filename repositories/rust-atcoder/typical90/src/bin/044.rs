fn main() {
    proconio::input! {
        n: usize,
        q: usize,
    }

    proconio::input! {
        mut a: [usize; n],
    }

    let mut base = 0;

    for _ in 0..q {
        proconio::input! {
            t: usize,
            mut x: usize,
            mut y: usize,
        }

        if x > 0 {
            x = ((x - 1) + base) % n;
        }
        if y > 0 {
            y = ((y - 1) + base) % n;
        }

        match t {
            1 => a.swap(x, y),
            2 => base = if base > 0 { (base - 1) % n } else { n - 1 },
            3 => println!("{}", a[x]),
            _ => panic!(),
        }
    }
}
