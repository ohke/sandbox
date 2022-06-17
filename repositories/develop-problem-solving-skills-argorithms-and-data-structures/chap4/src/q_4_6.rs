struct Solution;

impl Solution {
    pub fn solve(n: usize, w: u64, a: Vec<u64>) -> bool {
        let mut dp = vec![vec![Option::None; (w + 1) as usize]; n + 1];
        Self::_solve(n, w, &a, &mut dp)
    }

    fn _solve(i: usize, w: u64, a: &Vec<u64>, dp: &mut Vec<Vec<Option<bool>>>) -> bool {
        if i == 0 {
            if w == 0 {
                return true;
            } else {
                return false;
            }
        }

        if let Some(v) = dp[i][w as usize] {
            return v;
        }

        if Self::_solve(i - 1, w, a, dp) {
            dp[i][w as usize] = Some(true);
            return true;
        }

        if Self::_solve(i - 1, w - a[i - 1], a, dp) {
            dp[i][w as usize] = Some(true);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(4, 5, vec![1, 2, 4, 8]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve(4, 16, vec![1, 2, 4, 8]), false);
    }
}
