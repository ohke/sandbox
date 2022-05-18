fn solve(d: Vec<u64>, t: Vec<u64>) -> bool {
    let n = d.len();

    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((d[i], t[i]));
    }

    v.sort_by(|a, b| a.1.cmp(&b.1));

    let mut now = 0;
    for i in 0..n {
        now += v[i].0;
        if now > v[i].1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![2, 1, 1, 4, 3], vec![4, 9, 8, 9, 12]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![334, 334, 334], vec![1000, 1000, 1000]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                vec![
                    384, 1725, 170, 4, 2, 578, 702, 143, 1420, 24, 849, 76, 85, 444, 719, 470,
                    1137, 455, 110, 15, 368, 104, 3, 366, 7, 610, 152, 4, 292, 334
                ],
                vec![
                    8895, 9791, 1024, 11105, 6, 1815, 3352, 5141, 6980, 1602, 999, 7586, 5570,
                    4991, 11090, 10708, 4547, 9003, 9901, 8578, 3692, 1286, 4, 12143, 6649, 2374,
                    7324, 7042, 11386, 5720
                ]
            ),
            true
        );
    }
}
