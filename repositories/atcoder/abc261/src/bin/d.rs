use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize; n],
    }

    for _ in 0..m {
        input! {
            c: usize,
            y: usize,
        }
        x[c - 1] += y;
    }

    let mut g = Vec::new();
    for i in 0..n {
        let mut g_i = if i == 0 { x[i] } else { g[i - 1] + x[i] };

        g_i *= (n + 1) / (i + 2);

        g.push(g_i);
    }

    let mut max_i = 0;
    let mut max_v = 0;
    for i in 0..n {
        if g[i] > max_v {
            max_v = g[i];
            max_i = i;
        }
    }
}
