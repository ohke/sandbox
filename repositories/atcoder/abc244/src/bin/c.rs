use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    let mut a = vec![false; 2 * n + 2];
    let mut count = 0;
    loop {
        for i in 1..2 * n + 2 {
            if !a[i] {
                a[i] = true;
                println!("{}", i);
                stdout().flush().unwrap();
                break;
            }
        }

        count += 1;

        if count == 2 * n + 1 {
            println!("0");
            stdout().flush().unwrap();
            break;
        }

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let x = s.trim().parse::<usize>().unwrap();
        a[x] = true;

        count += 1;
    }
}
