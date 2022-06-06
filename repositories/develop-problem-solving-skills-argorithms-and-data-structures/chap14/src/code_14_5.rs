use std::collections::HashMap;

fn floyd_warshall(n: usize, e: Vec<(usize, usize, i64)>) -> Vec<Vec<Option<i64>>> {
    let mut graph = vec![HashMap::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
    }

    let mut dp = vec![vec![Option::None; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[i][j] = Some(0);
            } else if graph[i].contains_key(&j) {
                dp[i][j] = Some(graph[i][&j]);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dp[i][k].is_some() && dp[k][j].is_some() {
                    let challenger = dp[i][k].unwrap() + dp[k][j].unwrap();
                    if dp[i][j].is_none() {
                        dp[i][j] = Some(challenger);
                    } else {
                        dp[i][j] = Some(dp[i][j].unwrap().min(challenger));
                    }
                }
            }
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            floyd_warshall(4, vec![(0, 1, 1), (0, 2, 5), (1, 2, 3), (2, 3, -1),],),
            vec![
                vec![Some(0), Some(1), Some(4), Some(3)],
                vec![None, Some(0), Some(3), Some(2)],
                vec![None, None, Some(0), Some(-1)],
                vec![None, None, None, Some(0)],
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            floyd_warshall(
                4,
                vec![(0, 1, 1), (0, 2, 5), (1, 2, 3), (2, 3, -1), (3, 1, 1),],
            ),
            vec![
                vec![Some(0), Some(1), Some(4), Some(3)],
                vec![None, Some(0), Some(3), Some(2)],
                vec![None, Some(0), Some(0), Some(-1)],
                vec![None, Some(1), Some(4), Some(0)],
            ]
        );
    }
}
