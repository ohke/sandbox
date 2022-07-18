use proconio::input;

fn main() {
    input! {
        x_1: i32,
        y_1: i32,
    }

    input! {
        x_2: i32,
        y_2: i32,
    }

    input! {
        x_3: i32,
        y_3: i32,
    }

    let x = if x_1 == x_2 {
        x_3
    } else if x_1 == x_3 {
        x_2
    } else {
        x_1
    };

    let y = if y_1 == y_2 {
        y_3
    } else if y_1 == y_3 {
        y_2
    } else {
        y_1
    };

    println!("{} {}", x, y);
}
