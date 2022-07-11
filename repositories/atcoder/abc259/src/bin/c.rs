fn main() {
    proconio::input! {
        s: String,
        t: String,
    }

    let s: Vec<char> = s.chars().collect();
    let mut j = 0;
    for t_i in t.chars() {
        if j < s.len() && t_i == s[j] {
            j += 1;
            continue;
        } else if j >= 2 && t_i == s[j - 1] && s[j - 1] == s[j - 2] {
            continue;
        }

        println!("No");
        return;
    }

    if s.len() == j {
        println!("Yes");
    } else {
        println!("No");
    }
}
