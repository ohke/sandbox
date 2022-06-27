use superslice::Ext;

fn main() {
    proconio::input! {
        n: usize,
    }

    proconio::input! {
        mut a: [i64; n],
    }

    proconio::input! {
        q: usize,
    }

    let mut b = Vec::new();
    for _ in 0..q {
        proconio::input! {
            b_i: i64,
        }

        b.push(b_i);
    }

    a.sort();

    for b_i in b.iter() {
        let index = a.upper_bound(b_i);

        let ret = if index == 0 || index == a.len() - 1 {
            (a[index] - b_i).abs()
        } else if index == a.len() {
            (a[index - 1] - b_i).abs()
        } else {
            (a[index] - b_i).abs().min((a[index - 1] - b_i).abs())
        };

        println!("{}", ret);
    }
    
}
