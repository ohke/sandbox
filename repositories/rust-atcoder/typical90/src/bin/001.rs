fn main() {
    proconio::input! {
        n: usize,
        l: usize,
    }

    proconio::input! {
        k: usize,
    }

    proconio::input! {
        a: [usize; n],
    }

    let mut v = Vec::new();
    v.push(a[0]);
    for i in 1..a.len() {
        v.push(a[i] - a[i - 1]);
    }
    v.push(l - a[a.len() - 1]);

    let mut s = 0;

    let mut left = 0;
    let mut right = l;

    while left <= right {
        let mid = left + (right - left) / 2;

        let mut n_blocks = 0;
        let mut acc = 0;
        let mut score = std::usize::MAX;

        for v_i in v.iter() {
            acc += v_i;
            if acc >= mid {
                score = score.min(acc);
                acc = 0;
                n_blocks += 1;
            }
        }

        if n_blocks == k + 1 {
            s = s.max(score);
        }

        if left == right {
            break;
        } else if n_blocks >= k + 1 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    println!("{}", s);
}
