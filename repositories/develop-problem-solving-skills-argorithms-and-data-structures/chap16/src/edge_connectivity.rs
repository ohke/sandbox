use std::collections::HashSet;

fn dfs(
    graph: &Vec<HashSet<usize>>,
    s: usize,
    t: usize,
    visited: &mut HashSet<usize>,
) -> Option<Vec<(usize, usize)>> {
    if visited.contains(&s) {
        return None;
    }

    visited.insert(s);

    if graph[s].contains(&t) {
        return Some(vec![(s, t)]);
    }

    for e_i in graph[s].iter().cloned() {
        if let Some(mut p) = dfs(graph, e_i, t, visited) {
            p.insert(0, (s, e_i));
            return Some(p);
        }
    }

    None
}

fn edge_connectivity(n: usize, e: Vec<(usize, usize)>, s: usize, t: usize) -> usize {
    let mut graph = vec![HashSet::new(); n];
    for e_i in e.into_iter() {
        graph[e_i.0].insert(e_i.1);
    }
    println!("graph: {:?}", graph);

    let mut ret = 0;

    loop {
        if let Some(p) = dfs(&graph, s, t, &mut HashSet::new()) {
            println!("p: {:?}", p);

            // 残余グラフへの更新
            for e_i in p.iter() {
                graph[e_i.0].remove(&e_i.1);
                graph[e_i.1].insert(e_i.0);
            }

            println!("graph: {:?}", graph);

            ret += 1;
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
            edge_connectivity(
                12,
                vec![
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (1, 4),
                    (2, 4),
                    (2, 5),
                    (3, 5),
                    (4, 6),
                    (5, 7),
                    (6, 5),
                    (6, 8),
                    (6, 9),
                    (7, 9),
                    (7, 10),
                    (8, 11),
                    (9, 11),
                    (10, 11),
                ],
                0,
                11,
            ),
            2,
        )
    }
}
