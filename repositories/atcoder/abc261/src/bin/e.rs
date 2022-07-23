use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u32,
    }

    let mut t = Vec::new();
    let mut a = Vec::new();
    for _ in 0..n {
        input! {
            t_i: u8,
            a_i: u32,
        }

        t.push(t_i);
        a.push(a_i);
    }

    let mut x = c;
    for i in 0..n {
        for j in 0..(i + 1) {
            x = match t[j] {
                1 => x & a[j],
                2 => x | a[j],
                3 => x ^ a[j],
                _ => panic!(),
            };
        }

        println!("{}", x);
    }
}
