fn solve(a: Vec<i64>, b: Vec<i64>) -> u64 {
    use std::collections::HashMap;

    let mut m: HashMap<i64, u64> = HashMap::new();
    for a_i in a.iter() {
        let v = match m.get(a_i) {
            Some(v) => *v + 1,
            None => 1,
        };

        m.insert(*a_i, v);
    }

    let mut ret = 0;
    for b_j in b.iter() {
        if let Some(v) = m.get(b_j) {
            ret += *v;
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

    #[test]
    fn test_4() {
        assert_eq!(solve(vec![1, 1, 1], vec![1, 1]), 6);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve(vec![1, 2, 1], vec![1, 1, 2]), 5);
    }
}
