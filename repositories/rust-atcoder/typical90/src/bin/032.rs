use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
    }

    proconio::input! {
        a: [[usize; n]; n],
    }

    proconio::input! {
        m: usize,
    }

    let mut ng = vec![vec![false; n + 1]; n + 1];
    for _ in 0..m {
        proconio::input! {
            x_i: usize,
            y_i: usize,
        }

        ng[x_i][y_i] = true;
        ng[y_i][x_i] = true;
    }

    let mut min = std::usize::MAX;
    for npr in (1..(n + 1)).permutations(n) {
        let mut t = 0;
        let mut all_ok = true;

        for (j, i) in npr.iter().enumerate() {
            if j > 0 && ng[npr[j - 1]][*i] {
                all_ok = false;
                break;
            }
            t += a[*i - 1][j];
        }

        if all_ok {
            min = min.min(t);
        }
    }

    if min < std::usize::MAX {
        println!("{}", min);
    } else {
        println!("-1");
    }
}
