use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(
    graph: &Vec<HashMap<usize, usize>>,
    seen: &mut HashSet<usize>,
    from: usize,
    to: usize,
) -> Option<(Vec<(usize, usize)>, usize)> {
    if seen.contains(&from) {
        return None;
    }

    seen.insert(from);

    for (t, c) in graph[from].iter() {
        if *c == 0 {
            continue;
        }

        if *t == to {
            return Some((vec![(from, *t)], *c));
        }

        if let Some((mut path, flow)) = dfs(graph, seen, *t, to) {
            path.insert(0, (from, *t));
            return Some((path, flow.min(*c)));
        }
    }

    seen.remove(&from);

    None
}

fn floyd_warshall(n: usize, graph: &Vec<HashMap<usize, usize>>) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[i][j] = 0;
            } else if let Some(e_i) = graph[i].get(&j) {
                dp[i][j] = *e_i;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if i == j || dp[i][k] == usize::MAX || dp[k][j] == usize::MAX {
                    continue;
                }
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    dp
}

fn solve(n: usize, e: Vec<(usize, usize, usize, usize)>, s: usize, t: usize) -> usize {
    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
    }

    let mut dp = floyd_warshall(n, &graph);

    // TODO: フォードファルカーソンの実装
    // let mut flow_graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    // for i in 0..n {
    //     for j in 0..n {
    //         if i == j {
    //             continue;
    //         }

    //         if let Some(graph[i])

    //         flow_graph[e_i.0].insert(e_i.1, e_i.3);
    //         flow_graph[e_i.1].insert(e_i.0, 0);
    //     }
    // }

    let mut ret = 0;
    // loop {
    //     let mut seen = HashSet::new();

    //     if let Some((path, flow)) = dfs(&graph, &mut seen, s, t) {
    //         for e_i in path.into_iter() {
    //             let c = graph[e_i.0].get(&e_i.1).unwrap().clone();
    //             graph[e_i.0].insert(e_i.1, c - flow);

    //             // 残余グラフ更新
    //             let c = graph[e_i.1].get(&e_i.0).unwrap().clone();
    //             graph[e_i.1].insert(e_i.0, c + flow);
    //         }

    //         ret += flow;
    //     } else {
    //         break;
    //     }
    // }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(
                3,
                vec![
                    (0, 1, 1, 1),
                    (1, 2, 1, 1),
                    (0, 2, 1, 1),
                ]
                0,
                2,
            ),
            1
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(
                8,
                vec![
                    (0, 4, 2, 3),
                    (2, 7, 3, 6),
                    (7, 6, 1, 3),
                    (1, 6, 6, 4),
                    (2, 6, 5, 5),
                    (7, 2, 1, 3),
                    (4, 5, 3, 5),
                    (0, 6, 3, 2),
                    (3, 2, 2, 4),
                    (4, 3, 4, 3),
                    (1, 2, 2, 2),
                    (1, 7, 6, 5),
                    (5, 1, 1, 3),
                    (3, 1, 1, 6),
                    (5, 0, 4, 2),
                ],
                4,
                6,
            ),
            8,
        )
    }
}
