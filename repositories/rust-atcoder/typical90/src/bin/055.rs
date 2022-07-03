fn main() {
    proconio::input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }

    let mut ret = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if (a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p) == q {
                            ret += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ret);
}
