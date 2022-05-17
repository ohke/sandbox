fn solve(mut red: Vec<(i64, i64)>, mut blue: Vec<(i64, i64)>) -> u64 {
    let mut ret = 0;

    blue.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    red.sort_by(|a, b| (-a.1).partial_cmp(&(-b.1)).unwrap());

    for i in 0..blue.len() {
        for j in 0..red.len() {
            if blue[i].0 > red[j].0 && blue[i].1 > red[j].1 {
                red.remove(j);
                ret += 1;
                break;
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(vec![(2, 0), (3, 1), (1, 3)], vec![(4, 2), (0, 4), (5, 5)]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(vec![(0, 0), (1, 1), (5, 2)], vec![(2, 3), (3, 4), (4, 5)]),
            2
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(vec![(2, 2), (3, 3)], vec![(0, 0), (1, 1)]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            solve(
                vec![(0, 0), (7, 3), (2, 2), (4, 8), (1, 6)],
                vec![(8, 5), (6, 9), (5, 4), (9, 1), (3, 7)]
            ),
            5
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            solve(
                vec![(0, 0), (1, 1), (5, 5), (6, 6), (7, 7)],
                vec![(2, 2), (3, 3), (4, 4), (8, 8), (9, 9)]
            ),
            4
        );
    }
}
