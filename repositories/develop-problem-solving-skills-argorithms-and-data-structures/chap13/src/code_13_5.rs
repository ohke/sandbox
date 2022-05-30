fn solve(n: usize, e: Vec<(usize, usize)>) -> bool {
    use std::collections::VecDeque;

    #[derive(Clone, PartialEq)]
    enum Color {
        Gray,
        White,
        Black,
    }

    let mut graph = vec![vec![]; n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
        graph[e_i.1].push(e_i.0);
    }

    let mut colors = vec![Color::Gray; n];
    colors[0] = Color::White;

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while queue.len() > 0 {
        let i = queue.pop_front().unwrap();

        for j in graph[i].iter() {
            if colors[*j] == Color::Gray {
                colors[*j] = match colors[i] {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                    _ => panic!(),
                };
                queue.push_back(*j);
            } else if colors[i] == colors[*j] {
                return false;
            }
        }
    }

    true
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
