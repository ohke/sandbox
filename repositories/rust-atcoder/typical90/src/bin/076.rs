fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();

    let mut i = 0;
    let mut j = 1;
    // [a_i, a_j) の合計
    let mut current = a[i];
    loop {
        if current * 10 < sum {
            current += a[j];
            j = (j + 1) % n;
        } else if current * 10 > sum {
            current -= a[i];
            i += 1;
            if i == n {
                println!("No");
                break;
            }
        } else {
            println!("Yes");
            break;
        }
    }
}
