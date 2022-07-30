use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut e = vec![Vec::new(); n + 1];
    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }
        e[x].push(y);
    }

    let mut dp = vec![0; n + 1];

    for i in 1..(n + 1) {
        if dp[i] > 0 {
            continue;
        }

        let mut q: Vec<usize> = vec![i];
        while let Some(i) = q.pop() {
            for j in e[i].iter().cloned() {
                if dp[i] + 1 > dp[j] {
                    dp[j] = dp[i] + 1;
                    q.push(j);
                }
            }
        }
    }

    let result = dp.iter().max().unwrap();
    println!("{}", result);
}
