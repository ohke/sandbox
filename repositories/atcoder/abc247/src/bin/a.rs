fn main() {
    proconio::input! {
        mut s: String,
    }

    s.insert(0, '0');
    s.pop();

    println!("{}", &s);
}
