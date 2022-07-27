use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut b_i = 0;
    for a_i in 0..n {
        if b[b_i] == a[a_i] {
            b_i += 1;
            if b_i == m {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
