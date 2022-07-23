use proconio::input;

fn main() {
    input! {
        l_1: usize,
        r_1: usize,
        l_2: usize,
        r_2: usize,
    }

    let result = if l_1 > r_2 || l_2 > r_1 {
        0
    } else {
        let l = l_1.max(l_2);
        let r = r_1.min(r_2);
        r - l
    };

    println!("{}", result);
}
