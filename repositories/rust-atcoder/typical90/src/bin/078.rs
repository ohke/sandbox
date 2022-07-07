fn main() {
    proconio::input! {
        n: usize,
        m: usize,
    }

    let mut v = vec![0; n];

    for _ in 0..m {
        proconio::input! {
            a_i: usize,
            b_i: usize,
        }

        if a_i < b_i {
            v[b_i - 1] += 1;
        } else if a_i > b_i {
            v[a_i - 1] += 1;
        } else {
            panic!();
        }
    }

    let ret = v.iter().filter(|v_i| **v_i == 1).count();
    println!("{}", ret);
}
