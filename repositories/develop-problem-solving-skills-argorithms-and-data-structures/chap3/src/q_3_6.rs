fn solve(k: u32, n: u32) -> u32 {
    let mut ret = 0;

    let k = k as i64;
    let n = n as i64;
    for x in 0..k + 1 {
        if n < x {
            break;
        }

        for y in 0..k + 1 {
            let z = n - x - y;
            if z < 0 {
                break;
            } else if k < z {
                continue;
            } else {
                ret += 1;
            }
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(3, 3), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(2, 3), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(4, 3), 10);
    }
}
