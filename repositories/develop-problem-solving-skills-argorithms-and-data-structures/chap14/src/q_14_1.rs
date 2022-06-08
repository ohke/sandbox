use std::collections::HashSet;

fn rec(graph: &Vec<HashSet<usize>>, dp: &mut Vec<Option<usize>>, v: usize) -> usize {
    if let Some(d) = dp[v] {
        return d;
    }

    let mut d = 0;
    for next_v in graph[v].iter().cloned() {
        let next_d = rec(&graph, dp, next_v) + 1;
        if next_d > d {
            d = next_d;
        }
    }

    dp[v] = Some(d);
    
    d
}

fn solve(n: usize, e: Vec<(usize, usize)>) -> usize {
    let mut graph = vec![HashSet::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1);
    }

    let mut dp = vec![Option::None; n];

    for i in 0..n {
        dp[i] = Some(0.max(rec(&graph, &mut dp, i)))
    }

    dp.iter().map(|e| e.unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 3),
                    (4, 5),
                ],
            ),
            5
        );
    }
}
