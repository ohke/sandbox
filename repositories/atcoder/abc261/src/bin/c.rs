use im_rc::{HashMap, HashSet};
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }

        match map.get(&s) {
            Some(v) => {
                println!("{}({})", &s, v);
                map.insert(s, v + 1);
            }
            None => {
                println!("{}", &s);
                map.insert(s, 1);
            }
        }
    }
}
