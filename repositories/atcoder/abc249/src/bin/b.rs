use im_rc::HashSet;

fn main() {
    proconio::input! {
        s: String,
    }

    let mut set = HashSet::new();
    let mut upper = false;
    let mut lower = false;
    for c in s.trim().chars() {
        if !upper {
            upper = c.is_uppercase();
        }
        if !lower {
            lower = c.is_lowercase();
        }
        set.insert(c);
    }

    if upper && lower && set.len() == s.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
