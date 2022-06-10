use std::collections::{HashSet, VecDeque};

fn solve(n: usize, e: Vec<(usize, usize)>, s: usize, t: usize) -> Option<usize> {
    let n = n * 3;
    let s = s * 3;
    let t = t * 3;

    let mut graph = vec![HashSet::new(); n];
    for e_i in e.iter() {
        graph[e_i.0 * 3].insert(e_i.1 * 3 + 1);
        graph[e_i.0 * 3 + 1].insert(e_i.1 * 3 + 2);
        graph[e_i.0 * 3 + 2].insert(e_i.1 * 3);
    }

    let mut dist = vec![Option::None; n];

    let mut queue = VecDeque::new();
    queue.push_back((s, 0));

    while queue.len() > 0 {
        let (v, c) = queue.pop_front().unwrap();

        if dist[v].is_some() {
            continue;
        }

        dist[v] = Some(c);

        for e in graph[v].iter().cloned() {
            queue.push_back((e, c + 1));
        }
    }

    dist[t]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)], 0, 3),
            Some(3)
        );
    }
}
