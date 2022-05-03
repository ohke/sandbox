struct Solution;

impl Solution {
    pub fn solve(mut a: Vec<i32>) -> usize {
        let mut ret = 0;

        loop {
            let mut all = true;

            for i in 0..a.len() {
                if a[i] % 2 == 0 {
                    a[i] /= 2;
                } else {
                    all = false;
                    break;
                }
            }

            if !all {
                break;
            }

            ret += 1;
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve(vec![2, 4, 8]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve(vec![1, 4, 8]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::solve(vec![8, 8, 8]), 3);
    }
}
