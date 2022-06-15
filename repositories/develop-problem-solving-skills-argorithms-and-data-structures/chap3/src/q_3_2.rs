struct Solution;

impl Solution {
    pub fn solve(a: Vec<i32>, v: i32) -> usize {
        a.iter().filter(|&&a_i| a_i == v).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(vec![1, 1, 2, 1, 1, 2, 3, 2, 1], 1), 5);
    }
}
