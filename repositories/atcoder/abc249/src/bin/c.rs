use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = vec![vec![0; 26]; n];
    for i in 0..n {
        input! {
            s_i: String,
        }

        for c in s_i.chars() {
            a[i][(c as u32 - 'a' as u32) as usize] = 1;
        }
    }

    let mut count = 0;
    for mut m in 0..2_i32.pow(n as u32) {
        if m.count_ones() < k as u32 {
            continue;
        }

        let mut c = vec![0; 26];
        let mut i = 0;
        while m > 0 {
            if m & 1 == 1 {
                for j in 0..26 {
                    c[j] += a[i][j];
                }
            }
            m >>= 1;
            i += 1;
        }

        let count_m = c.iter().filter(|c_i| **c_i == k).count();
        count = count.max(count_m);
    }

    println!("{}", count);
}
