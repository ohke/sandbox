fn main() {
    proconio::input! {
        n: i64,
    }

    proconio::input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ret = std::i64::MAX;
    for x in 0..10000 {
        let r = n - a*x;
        if r < 0 {
            break;
        }

        for y in 0..10000 {
            let r = r - b*y;
            if r < 0 {
                break;
            }

            if r % c == 0 {
                ret = ret.min(x + y + r / c);
            }
        }
    }

    println!("{}", ret);
}
