fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let mut l_takahashi = (x / (a + c)) * a * b;
    l_takahashi += a.min(x % (a + c)) * b;

    let mut l_aoki = (x / (d + f)) * d * e;
    l_aoki += d.min(x % (d + f)) * e;

    if l_takahashi > l_aoki {
        println!("Takahashi");
    } else if l_aoki > l_takahashi {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
