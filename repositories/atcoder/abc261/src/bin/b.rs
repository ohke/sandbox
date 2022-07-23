use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec!['-'; n]; n];
    for i in 0..n {
        input! {
            a_i: String,
        }

        for (j, c) in a_i.chars().enumerate() {
            a[i][j] = c;
        }
    }

    for i in 0..n {
        for j in 0..i {
            if a[i][j] == 'W' && a[j][i] != 'L'
                || a[i][j] == 'L' && a[j][i] != 'W'
                || a[i][j] == 'D' && a[j][i] != 'D'
            {
                println!("incorrect");
                return;
            }
        }
    }

    println!("correct");
}
