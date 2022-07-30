use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;

    let mut v: Vec<(usize, usize)> = Vec::new();
    v.push((0, 0)); // 番兵

    for a_i in a.iter().cloned() {
        let (v_l, v_l_count) = v.last().unwrap().clone();

        if a_i != v_l {
            v.push((a_i, 1));
            count += 1;
        } else if v_l_count + 1 < v_l {
            v.pop();
            v.push((a_i, v_l_count + 1));
            count += 1;
        } else {
            v.pop();
            count -= a_i - 1;
        }

        println!("{}", count);
    }
}
