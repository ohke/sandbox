fn main() {
    proconio::input! {
        n: u32,
    }

    for i in 0..2_u32.pow(n) {
        let mut s = Vec::new();
        let mut r = 0;
        let mut ok = true;

        for j in 0..n {
            let mask = 2_u32.pow(n - j - 1);
            if i & mask == 0 {
                s.push('(');
                r += 1;
            } else {
                s.push(')');
                r -= 1;
            }

            if r < 0 {
                ok = false;
                break;
            }
        }

        if ok && r == 0 {
            let s: String = s.into_iter().collect();
            println!("{}", &s);
        }
    }
}
