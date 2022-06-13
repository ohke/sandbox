fn q_15_2(n: usize, mut e: Vec<(usize, usize, usize)>) -> f64 {
    #[derive(Debug)]
    struct UFTree {
        roots: Vec<usize>,
        sizes: Vec<usize>,
    }

    impl UFTree {
        pub fn new(n: usize) -> UFTree {
            UFTree {
                roots: (0..n).collect(),
                sizes: vec![1; n],
            }
        }

        pub fn root(&mut self, i: usize) -> usize {
            if i == self.roots[i] {
                return i;
            }

            self.roots[i] = self.root(self.roots[i]);
            self.roots[i]
        }

        pub fn issame(&mut self, i: usize, j: usize) -> bool {
            self.root(i) == self.root(j)
        }

        pub fn unite(&mut self, i: usize, j: usize) {
            if self.issame(i, j) {
                return;
            }

            if self.sizes[i] > self.sizes[j] {
                let root_j = self.root(j);
                self.roots[root_j] = self.roots[i];
            } else {
                let root_i = self.root(i);
                self.roots[root_i] = self.roots[j];
            }

            let new_size = self.sizes[i] + self.sizes[j];
            self.sizes[i] = new_size;
            self.sizes[j] = new_size;
        }
    }

    let mut tree = UFTree::new(n);

    let mut count = 0;
    let mut prev_w = 0;
    e.sort_by_key(|e_i| e_i.2);
    for e_i in e.iter() {
        if tree.issame(e_i.0, e_i.1) {
            continue;
        }

        tree.unite(e_i.0, e_i.1);

        count += 1;
        if count > (n - 1) / 2 {
            if n % 2 == 0 {
                return e_i.2 as f64;
            } else {
                return (e_i.2 + prev_w) as f64 / 2.0;
            }
        }

        prev_w = e_i.2;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            q_15_2(
                8,
                vec![
                    (0, 3, 5),
                    (0, 5, 6),
                    (0, 7, 3),
                    (1, 3, 8),
                    (1, 4, 4),
                    (1, 6, 3),
                    (2, 4, 9),
                    (2, 5, 10),
                    (2, 7, 5),
                    (3, 7, 6),
                    (4, 6, 2),
                    (6, 7, 7),
                ]
            ),
            5.0
        )
    }
}
