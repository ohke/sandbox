use std::collections::BTreeSet;

fn solve(a: Vec<u64>, k: usize) -> Vec<u64> {
    let mut set = BTreeSet::new();
    let mut ret = Vec::new();

    for (i, a_i) in a.iter().enumerate() {
        set.insert(*a_i);
        if i >= k - 1 {
            let v = set.iter().skip(k - 1).next().unwrap();
            ret.push(*v);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![2, 0, 3, 5, 1, 4], 3), vec![3, 3, 2, 2]);
    }
}
