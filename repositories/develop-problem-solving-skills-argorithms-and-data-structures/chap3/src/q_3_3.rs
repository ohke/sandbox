struct Solution;

impl Solution {
    pub fn solve(a: Vec<i32>) -> i32 {
        let mut ret = i32::MAX;
        let mut min = i32::MAX;

        for a_i in a.into_iter() {
            if min > a_i {
                ret = min;
                min = a_i;
            } else if ret > a_i {
                ret = a_i;
            }
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(vec![3, 9, 1, 4, 5, 2, 8, 0, 7, 6]), 1);
    }
}
