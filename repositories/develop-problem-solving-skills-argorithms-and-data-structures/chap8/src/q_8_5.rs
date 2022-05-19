fn solve(a: Vec<i64>, b: Vec<i64>) -> u64 {
    use std::collections::HashSet;

    let s: HashSet<i64> = a.iter().cloned().collect();

    let mut ret = 0;
    for b_j in b.iter() {
        if s.contains(b_j) {
            ret += 1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![1, 2, 3], vec![1, 10, 2]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![1, 2, 3], vec![]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![], vec![1, 2, 3]), 0);
    }
}
