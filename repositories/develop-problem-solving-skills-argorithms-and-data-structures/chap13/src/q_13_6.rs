use std::collections::HashSet;

fn dfs(
    v: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    passed: &mut HashSet<usize>,
) -> Option<Vec<usize>> {
    if passed.contains(&v) {
        return Some(vec![v]);
    }

    if seen[v] {
        return None;
    }

    passed.insert(v);

    for neighbor in graph[v].iter().cloned() {
        if let Some(mut cyclic) = dfs(neighbor, graph, seen, passed) {
            if passed.contains(&cyclic.last().unwrap()) {
                cyclic.insert(0, v);
            }
            passed.remove(&v);

            return Some(cyclic);
        } else {
            passed.remove(&v);
        }
    }

    seen[v] = true;

    None
}

fn solve(n: usize, e: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
    }

    let mut seen = vec![false; n];

    for v in 0..n {
        let mut passed = HashSet::new();

        let ret = dfs(v, &graph, &mut seen, &mut passed);
        if ret.is_some() {
            return ret;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(
                8,
                vec![
                    (0, 5),
                    (1, 3),
                    (1, 6),
                    (2, 5),
                    (2, 7),
                    (3, 0),
                    (3, 7),
                    (4, 1),
                    (4, 2),
                    (4, 6),
                    (6, 7),
                    (7, 0)
                ]
            ),
            None,
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(
                8,
                vec![
                    (0, 5),
                    (1, 3),
                    (1, 6),
                    (5, 2),
                    (2, 7),
                    (3, 0),
                    (3, 7),
                    (4, 1),
                    (4, 2),
                    (4, 6),
                    (6, 7),
                    (7, 0)
                ]
            ),
            Some(vec![0, 5, 2, 7, 0]),
        );
    }
}
