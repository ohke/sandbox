fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    proconio::input! {
        r: usize,
        c: usize,
    }

    let mut ret = 0;
    // 左
    if c > 1 {
        ret += 1;
    }
    // 上
    if r > 1 {
        ret += 1;
    }
    // 右
    if c < w {
        ret += 1;
    }
    // 下
    if r < h {
        ret += 1;
    }

    println!("{}", ret);
}
