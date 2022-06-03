fn solve(n: usize, e: Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
    }

    let mut order = vec![Option::None; n];

    for i in 0..n {
        if order[i].is_some() {
            continue;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((i, 0 as usize));

        while let Some((j, o)) = queue.pop_front() {
            if let Some(current_o) = order[j] {
                if current_o > o {
                    continue;
                }
            }

            order[j] = Some(o);
            let next_o = o + 1;
            for next in graph[j].iter().cloned() {
                queue.push_back((next, next_o));
            }
        }
    }

    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push((i, order[i].unwrap()));
    }

    result.sort_by_key(|e| e.1);

    result.iter().map(|e| e.0).collect()
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
            vec![4, 1, 6, 2, 3, 7, 0, 5]
        );
    }
}
