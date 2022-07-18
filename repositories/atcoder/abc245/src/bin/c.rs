use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut a_ok = true;
    let mut b_ok = true;
    let mut next_a_ok = false;
    let mut next_b_ok = false;
    for i in 1..n {
        if a_ok {
            if (a[i - 1] - a[i]).abs() <= k {
                next_a_ok = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                next_b_ok = true;
            }
        }
        if b_ok {
            if (b[i - 1] - a[i]).abs() <= k {
                next_a_ok = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                next_b_ok = true;
            }
        }

        a_ok = next_a_ok;
        b_ok = next_b_ok;
        next_a_ok = false;
        next_b_ok = false;
    }

    let result = if a_ok || b_ok { "Yes" } else { "No" };

    println!("{}", result);
}
