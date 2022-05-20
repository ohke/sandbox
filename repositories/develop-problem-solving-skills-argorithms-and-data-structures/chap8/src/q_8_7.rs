fn solve(a: Vec<i64>, b: Vec<i64>, k: i64) -> bool {
    use std::collections::HashSet;

    let s: HashSet<i64> = a.iter().cloned().collect();

    for b_j in b {
        if s.contains(&(k - b_j)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![1, 2, 3], vec![1, 2, 3], 3), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![1, 2, 3], vec![1, 2, 3], 7), false);
    }
}
