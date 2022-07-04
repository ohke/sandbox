fn main() {
    proconio::input! {
        q: usize,
    }

    let mut x = Vec::new();
    for _ in 0..q {
        proconio::input! {
            t_i: usize,
            x_i: usize,
        }

        match t_i {
            1 => x.insert(0, x_i),
            2 => x.push(x_i),
            3 => println!("{}", x[x_i - 1]),
            _ => panic!(),
        }
    }
}
