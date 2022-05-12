fn lower_bound(a: &Vec<u64>, v: u64) -> Option<usize> {
    let mut left = 0;
    let mut right = a.len() - 1;

    if a[right] < v {
        return None;
    } else if v <= a[left] {
        return Some(0);
    }

    while left <= right {
        let mid = left + (right - left) / 2;
        if v <= a[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    Some(left)
}

fn solve(a: Vec<u64>, m: u64) -> u64 {
    use std::collections::HashSet;

    let mut a = a;
    a.push(0);

    let mut s = HashSet::new();
    for i in 0..a.len() {
        for j in 0..a.len() {
            s.insert(a[i] + a[j]);
        }
    }

    let mut s: Vec<u64> = s.into_iter().collect();
    s.sort();

    let mut ret = 0;

    for i in 0..s.len() {
        let j = lower_bound(&s, m - s[i]);

        let challenger = s[i]
            + match j {
                Some(j) => {
                    if j == 0 {
                        continue;
                    } else {
                        s[j - 1]
                    }
                }
                None => s[s.len() - 1],
            };

        if ret < challenger {
            ret = challenger;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![3, 4], 16), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![3, 4], 17), 16);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![3, 4], 18), 16);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(vec![3, 4], 11), 10);
    }
}
