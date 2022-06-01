fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[v] = true;

    for next_v in graph[v].iter() {
        if seen[*next_v] {
            continue;
        }

        dfs(*next_v, graph, seen);
    }
}

fn solve(n: usize, e: Vec<(usize, usize)>) -> usize {
    let mut ret = 0;

    let mut graph = vec![Vec::new(); n];
    for e_i in e.iter() {
        graph[e_i.0].push(e_i.1);
        graph[e_i.1].push(e_i.0);
    }

    let mut seen = vec![false; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }

        ret += 1;

        dfs(i, &graph, &mut seen);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(7, vec![(0, 1), (1, 2), (1, 4), (3, 5), (4, 6)]), 2);
    }
}
