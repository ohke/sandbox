fn main() {
    proconio::input! {
        n: usize,
    }

    let mut acc_1 = vec![0];
    let mut acc_2 = vec![0];
    for i in 1..(n + 1) {
        proconio::input! {
            c_i: usize,
            p_i: usize,
        }

        if c_i == 1 {
            acc_1.push(acc_1[i - 1] + p_i);
            acc_2.push(acc_2[i - 1]);
        } else {
            acc_1.push(acc_1[i - 1]);
            acc_2.push(acc_2[i - 1] + p_i);
        }
    }

    proconio::input! {
        q: usize,
    }

    for _ in 0..q {
        proconio::input! {
            l_j: usize,
            r_j: usize,
        }

        println!(
            "{} {}",
            acc_1[r_j] - acc_1[l_j - 1],
            acc_2[r_j] - acc_2[l_j - 1],
        );
    }
}
