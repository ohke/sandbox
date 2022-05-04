struct Solution;

impl Solution {
    pub fn solve(k: u64) -> u64 {
        Solution::solve_(k, 0)
    }

    fn solve_(k: u64, mut current: u64) -> u64 {
        if k < current {
            return 0;
        }

        let mut ret = 0;

        let s = current.to_string();
        if s.contains("3") && s.contains("5") && s.contains("7") {
            ret += 1;
        }

        current *= 10;
        ret += Solution::solve_(k, current + 3);
        ret += Solution::solve_(k, current + 5);
        ret += Solution::solve_(k, current + 7);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(575), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve(3600), 13);
    }
}
