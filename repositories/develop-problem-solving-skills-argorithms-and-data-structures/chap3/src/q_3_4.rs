struct Solution;

impl Solution {
    pub fn solve(a: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for a_i in a.into_iter() {
            if min > a_i {
                min = a_i;
            }
            if max < a_i {
                max = a_i;
            }
        }

        max - min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(vec![3, 9, 1, 4, 5, 2, 8, 0, 7, 6]), 9);
    }
}
