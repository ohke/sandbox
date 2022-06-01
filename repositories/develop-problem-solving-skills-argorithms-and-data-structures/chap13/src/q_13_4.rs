fn bfs_shortest(n: usize, graph: &Vec<Vec<usize>>, s: usize, g: usize) -> usize {
    let mut cost = vec![Option::None; n];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((s, 0));

    while let Some((v, c)) = queue.pop_back() {
        if cost[v].is_some() {
            continue;
        }

        cost[v] = Some(c);
        
        for neighbor in graph[v].iter().cloned() {
            queue.push_back((neighbor, c + 1));
        }
    }

    cost[g].unwrap()
}

fn solve(n: usize, e: Vec<(usize, usize)>, s: usize, g: usize) -> usize {
    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
        graph[e_i.1].push(e_i.0);
    }

    bfs_shortest(n, &graph, s, g)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ooo 012
    // gxo 3-4
    // oos 567
    #[test]
    fn test_1() {
        assert_eq!(solve(8, vec![(0, 1), (0, 3), (1, 2), (2, 4), (3, 5), (4, 7), (5, 6), (6, 7)], 7, 3), 3);
    }

    // ooo 012
    // gxs 3-4
    // ooo 567
    #[test]
    fn test_2() {
        assert_eq!(solve(8, vec![(0, 1), (0, 3), (1, 2), (2, 4), (3, 5), (4, 7), (5, 6), (6, 7)], 4, 3), 4);
    }
}
