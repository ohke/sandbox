use std::collections::HashMap;

fn solve(n: usize, e: Vec<(usize, usize, i64)>) -> Option<i64> {
    let mut graph = vec![HashMap::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
    }

    let mut cost: Vec<Option<i64>> = vec![Option::None; n];
    cost[0] = Some(0);

    let mut updated = Vec::new();
    updated.push(0);

    for _ in 0..n {
        let mut new_updated = Vec::new();

        for i in updated.iter().cloned() {
            for e in graph[i].iter() {
                let new_cost = *e.1 + cost[i].unwrap();
                if cost[*e.0].is_none() || new_cost > cost[*e.0].unwrap() {
                    cost[*e.0] = Some(new_cost);
                    new_updated.push(*e.0);
                }
            }
        }

        if new_updated.len() == 0 {
            return cost[n - 1];
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
        assert_eq!(solve(3, vec![(0, 1, 4), (1, 2, 3), (0, 2, 5),],), Some(7),);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(2, vec![(0, 1, 1), (1, 0, 1),],), None,);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                6,
                vec![
                    (0, 1, -1000000000),
                    (1, 2, -1000000000),
                    (2, 3, -1000000000),
                    (3, 4, -1000000000),
                    (4, 5, -1000000000),
                ],
            ),
            Some(-5000000000),
        );
    }
}
