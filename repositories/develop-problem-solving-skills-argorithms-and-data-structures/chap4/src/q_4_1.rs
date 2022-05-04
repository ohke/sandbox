struct Solution;

impl Solution {
    pub fn solve(n: u64) -> u64 {
        if n == 0 || n == 1 {
            return 0u64;
        } else if n == 2 {
            return 1u64;
        }

        Solution::solve(n - 1) + Solution::solve(n - 2) + Solution::solve(n - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(8), 24);
    }
}
