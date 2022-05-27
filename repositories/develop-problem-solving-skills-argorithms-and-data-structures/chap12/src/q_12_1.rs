use std::collections::HashMap;

fn solve(mut a: Vec<u64>) -> Vec<u64> {
    let mut m = HashMap::new();

    let cloned_a = a.clone();
    a.sort();

    for (i, a_i) in a.iter().enumerate() {
        m.insert(*a_i, i as u64);
    }

    cloned_a.iter().map(|a_i| *m.get(a_i).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![2, 3, 1, 4]), vec![1, 2, 0, 3]);
    }
}
