fn main() {
    proconio::input! {
        _n: usize,
        s: String,
    }

    let mut a = Vec::new();
    a.push(0);
    let mut b = Vec::new();
    b.push(0);

    let mut ret = 0;
    let mut acc = 1;
    for (i, c) in s.chars().enumerate() {
        if c == 'o' {
            a.push(acc);
            b.push(b[i]);
        } else {
            a.push(a[i]);
            b.push(acc);
        }

        acc += 1;

        ret += a[i + 1].min(b[i + 1]);
    }

    println!("{}", ret);
}
