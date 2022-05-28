fn solve(a: Vec<u64>, b: Vec<u64>, m: u64) -> u64 {
    let mut array = Vec::new();
    for i in 0..a.len() {
        array.push((a[i], b[i]));
    }

    array.sort_by_key(|elem| elem.0);

    let mut ret = 0;
    let mut rest = m;
    for i in 0..a.len() {
        if rest <= array[i].1 {
            ret += array[i].0 * rest;
            break;
        } else {
            ret += array[i].0 * array[i].1;
            rest -= array[i].1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(vec![4, 2], vec![9, 4], 5), 12);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(vec![6, 2, 3, 7], vec![18, 5, 10, 9], 30), 130);
    }
}
