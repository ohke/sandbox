struct Solution;

impl Solution {
    pub fn solve(s: String) -> u64 {
        let mut ret = 0;

        let mut mask: u64 = 0;
        let max = 2usize.pow((s.len() - 1) as u32) as u64;

        while mask < max {
            let mut m = mask;
            let mut start = 0;
            let mut end = 1;

            for _ in 0..s.len() - 1 {
                if m & 1 == 1 {
                    ret += &s[start..end].parse().unwrap();
                    start = end;
                }
                end += 1;
                m >>= 1;
            }
            ret += &s[start..end].parse().unwrap();

            mask += 1;
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve("125".to_string()), 176);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve("1".to_string()), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::solve("1234".to_string()),
            1234 + (123 + 4)
                + (12 + 34)
                + (12 + 3 + 4)
                + (1 + 234)
                + (1 + 23 + 4)
                + (1 + 2 + 34)
                + (1 + 2 + 3 + 4)
        );
    }
}
