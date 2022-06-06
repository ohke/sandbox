use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn dijkstra_with_heap(n: usize, e: Vec<(usize, usize, usize)>, s: usize) -> Vec<Option<usize>> {
    let mut graph = vec![HashMap::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].insert(e_i.1, e_i.2);
    }

    let mut cost = vec![Option::None; n];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));

    while let Some((Reverse(c), v)) = heap.pop() {
        if cost[v].is_some() {
            continue;
        }

        cost[v] = Some(c);

        for (next_v, next_w) in graph[v].iter() {
            heap.push((Reverse(*next_w + c), *next_v));
        }
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            dijkstra_with_heap(
                6,
                vec![
                    (0, 1, 3),
                    (0, 2, 5),
                    (1, 2, 4),
                    (1, 3, 12),
                    (2, 3, 9),
                    (2, 4, 4),
                    (3, 5, 2),
                    (4, 3, 7),
                    (4, 5, 8),
                ],
                0
            ),
            vec![Some(0), Some(3), Some(5), Some(14), Some(9), Some(16)],
        );
    }
}
