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

fn ford_fulkerson(n: usize, e: Vec<(usize, usize, usize)>, s: usize, t: usize) -> usize {
    let mut graph = vec![HashMap::new(); n];
    for e_i in e.into_iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
        graph[e_i.1].insert(e_i.0, 0);
    }

    let mut ret = 0;

    loop {
        let mut seen = HashSet::new();

        if let Some((path, flow)) = dfs(&graph, &mut seen, s, t) {
            for e_i in path.into_iter() {
                let c = graph[e_i.0].get(&e_i.1).unwrap().clone();
                graph[e_i.0].insert(e_i.1, c - flow);

                // 残余グラフ更新
                let c = graph[e_i.1].get(&e_i.0).unwrap().clone();
                graph[e_i.1].insert(e_i.0, c + flow);
            }

            ret += flow;
        } else {
            break;
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
            ford_fulkerson(
                6,
                vec![
                    (0, 1, 5),
                    (0, 3, 5),
                    (1, 2, 4),
                    (2, 4, 56),
                    (2, 5, 9),
                    (3, 2, 3),
                    (3, 4, 9),
                    (4, 5, 2)
                ],
                0,
                5
            ),
            9
        )
    }
}
