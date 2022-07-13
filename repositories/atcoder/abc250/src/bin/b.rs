fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let n_columns = b * n;
    for v in 0..(a * b * n * n) {
        let i = v / n_columns;
        let j = v % n_columns;

        let c = if ((i / a) % 2 == 0) ^ ((j / b) % 2 == 0) {
            '#'
        } else {
            '.'
        };

        print!("{}", c);
        if j == n_columns - 1 {
            println!("");
        }
    }
}
