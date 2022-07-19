use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
    }

    let mut x = 0;
    let mut y = 0;
    let mut v_x = 1;
    let mut v_y = 0;
    for c in t.chars() {
        match c {
            'S' => {
                x += v_x;
                y += v_y;
            }
            'R' => {
                if v_x == 1 {
                    v_x = 0;
                    v_y = -1;
                } else if v_y == -1 {
                    v_x = -1;
                    v_y = 0;
                } else if v_x == -1 {
                    v_x = 0;
                    v_y = 1;
                } else {
                    v_x = 1;
                    v_y = 0;
                }
            }
            _ => panic!(),
        }
    }

    println!("{} {}", x, y);
}
