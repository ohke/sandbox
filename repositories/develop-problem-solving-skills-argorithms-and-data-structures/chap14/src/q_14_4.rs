use std::collections::HashMap;
use std::collections::VecDeque;

fn solve(h: usize, w: usize, m: &str) -> bool {
    let mut graph = vec![HashMap::new(); h * w];

    let mut s = 0;
    let mut g = 0;

    let mut map = vec![vec!['-'; w]; h];
    for (index, ch) in m.chars().filter(|c| *c != '\n').enumerate() {
        map[index / w][index % w] = ch;
    }

    for i in 0..h {
        for j in 0..w {
            if map[i][j] == 's' {
                s = w * i + j;
            } else if map[i][j] == 'g' {
                g = w * i + j;
            }

            if i > 0 {
                let next_ch = map[i - 1][j];
                let cost = if next_ch == '#' { 1 } else { 0 };
                graph[w * i + j].insert(w * (i - 1) + j, cost);
            }
            if i < h - 1 {
                let next_ch = map[i + 1][j];
                let cost = if next_ch == '#' { 1 } else { 0 };
                graph[w * i + j].insert(w * (i + 1) + j, cost);
            }
            if j > 0 {
                let next_ch = map[i][j - 1];
                let cost = if next_ch == '#' { 1 } else { 0 };
                graph[w * i + j].insert(w * i + (j - 1), cost);
            }
            if j < w - 1 {
                let next_ch = map[i][j + 1];
                let cost = if next_ch == '#' { 1 } else { 0 };
                graph[w * i + j].insert(w * i + (j + 1), cost);
            }
        }
    }

    let mut dist = vec![usize::MAX; h * w];
    dist[s] = 0;

    let mut queue = VecDeque::new();
    queue.push_front(s);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();

        for (next_v, c) in graph[v].iter() {
            if *c == 1 && dist[*next_v] > (dist[v] + 1) {
                dist[*next_v] = dist[v] + 1;
                queue.push_back(*next_v);
            } else if *c == 0 && dist[*next_v] > dist[v] {
                dist[*next_v] = dist[v];
                queue.push_front(*next_v);
            }
        }
    }

    println!("dist: {:?}", dist);

    dist[g] <= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(
                10,
                10,
                "s.........\n\
                 #########.\n\
                 #.......#.\n\
                 #..####.#.\n\
                 ##....#.#.\n\
                 #####.#.#.\n\
                 g##.#.#.#.\n\
                 ###.#.#.#.\n\
                 ###.#.#.#.\n\
                 #.....#...",
            ),
            true,
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(1, 10, "s..####..g"), false);
    }
}
