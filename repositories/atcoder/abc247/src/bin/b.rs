use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = Vec::with_capacity(n);
    let mut t = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            s_i: String,
            t_i: String,
        }

        s.push(s_i.clone());
        t.push(t_i.clone());
    }

    for i in 0..n {
        let nickname = s[i].clone();
        let mut s_ok = true;
        for j in 0..n {
            if j == i {
                continue;
            }
            if nickname == s[j] || nickname == t[j] {
                s_ok = false;
                break;
            }
        }

        let mut t_ok = true;
        if !s_ok {
            let nickname = t[i].clone();
            for j in 0..n {
                if j == i {
                    continue;
                }
                if nickname == s[j] || nickname == t[j] {
                    t_ok = false;
                    break;
                }
            }
        }

        if !s_ok && !t_ok {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
