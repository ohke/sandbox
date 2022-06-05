use std::collections::HashMap;

fn bellman_ford(n: usize, e: Vec<(usize, usize, i64)>, s: usize) -> Option<Vec<i64>> {
    let mut graph = vec![HashMap::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
    }

    let mut cost = vec![i64::MAX; n];
    cost[s] = 0;

    let mut updated = vec![];
    updated.push(s);

    for _ in 0..n {
        let mut new_updated = vec![];

        for i in updated.iter().cloned() {
            for (j, w) in graph[i].iter() {
                let new_cost = cost[i] + *w;
                if new_cost < cost[*j] {
                    cost[*j] = new_cost;
                    new_updated.push(*j);
                }
            }
        }

        if new_updated.len() == 0 {
            return Some(cost);
        }

        updated = new_updated;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            bellman_ford(
                6,
                vec![
                    (0, 1, 3),
                    (0, 3, 100),
                    (1, 2, 50),
                    (1, 3, 57),
                    (1, 4, -4),
                    (2, 3, -10),
                    (2, 4, -5),
                    (2, 5, 100),
                    (3, 1, -5),
                    (4, 2, 57),
                    (4, 3, 25),
                    (4, 5, 8)
                ],
                0
            ),
            Some(vec![0, 3, 53, 24, -1, 7])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            bellman_ford(
                6,
                vec![
                    (0, 1, 3),
                    (0, 3, 100),
                    (1, 2, 50),
                    (1, 3, 57),
                    (1, 4, -4),
                    (2, 3, -10),
                    (2, 4, -5),
                    (2, 5, 100),
                    (3, 1, -5),
                    (4, 2, 57),
                    (4, 3, -1),
                    (4, 5, 8)
                ],
                0
            ),
            None,
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            bellman_ford(3, vec![(0, 1, 1), (1, 2, 1)], 0),
            Some(vec![0, 1, 2]),
        );
    }
}
