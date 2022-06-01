fn bfs_bipartite(n: usize, graph: &Vec<Vec<usize>>) -> bool {
    #[derive(Clone, PartialEq)]
    enum Color {
        Gray,
        White,
        Black,
    }

    let mut color = vec![Color::Gray; n];

    let mut queue = std::collections::VecDeque::new();

    queue.push_back((0, Color::White));

    while let Some(this) = queue.pop_front() {
        for neighbor in graph[this.0].iter().cloned() {
            let neighbor_color = match this.1 {
                Color::White => Color::Black,
                Color::Black => Color::White,
                _ => panic!(),
            };

            color[this.0] = this.1.clone();

            if color[neighbor] == Color::Gray {
                queue.push_back((neighbor, neighbor_color));
            } else if color[neighbor] == this.1 {
                return false;
            }
        }
    }

    true
}

fn solve(n: usize, e: Vec<(usize, usize)>) -> bool {
    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
        graph[e_i.1].push(e_i.0);
    }

    bfs_bipartite(n, &graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(5, vec![(0, 1), (0, 3), (1, 2), (3, 4)]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(5, vec![(0, 1), (0, 2), (0, 3), (1, 2), (3, 4)]),
            false
        );
    }
}
