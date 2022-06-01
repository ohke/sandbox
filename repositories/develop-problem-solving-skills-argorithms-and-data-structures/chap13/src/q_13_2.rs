fn bfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    let mut queue = std::collections::VecDeque::new();

    queue.push_back(v);

    while let Some(v) = queue.pop_front() {
        seen[v] = true;
        for neighbor in graph[v].iter().cloned() {
            if !seen[neighbor] {
                queue.push_back(neighbor);
            }
        }
    }
}

fn solve(n: usize, e: Vec<(usize, usize)>, s: usize, t: usize) -> bool {
    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
        graph[e_i.1].push(e_i.0);
    }

    let mut seen = vec![false; n];

    bfs(s, &graph, &mut seen);

    seen[t]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(5, vec![(0, 1), (0, 2), (1, 3), (2, 4)], 3, 4), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(5, vec![(0, 1), (1, 3), (2, 4)], 3, 4), false);
    }
}
