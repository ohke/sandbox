fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    let mut count_a = vec![0 as usize; 46];
    let mut count_b = vec![0 as usize; 46];
    let mut count_c = vec![0 as usize; 46];
    for i in 0..n {
        count_a[a[i] % 46] += 1;
        count_b[b[i] % 46] += 1;
        count_c[c[i] % 46] += 1;
    }

    let mut ret = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ret += count_a[i] * count_b[j] * count_c[k];
                }
            }
        }
    }

    println!("{}", ret);
}
