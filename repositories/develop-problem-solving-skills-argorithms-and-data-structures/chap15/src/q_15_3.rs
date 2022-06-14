fn q_15_3(n: usize, mut e: Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    #[derive(Debug)]
    struct UFTree {
        n: usize,
        roots: Vec<usize>,
        sizes: Vec<usize>,
    }

    impl UFTree {
        pub fn new(n: usize) -> UFTree {
            UFTree {
                n,
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

            self.n -= 1;
        }
    }

    let mut ret = Vec::new();

    // まずは普通に最小全域木を求める
    let mut tree = UFTree::new(n);
    let mut weight = 0;

    e.sort_by_key(|e_i| e_i.2);
    let mut min_e = Vec::new();
    for (i, e_i) in e.iter().enumerate() {
        if tree.issame(e_i.0, e_i.1) {
            continue;
        }

        tree.unite(e_i.0, e_i.1);
        weight += e_i.2;

        min_e.push(i);
    }

    // 次に最小全域木を構成する辺を抜いてみて、2本の木になる or 木の重みが増える なら必要な辺
    for dropped_i in min_e.into_iter() {
        let mut tree = UFTree::new(n);
        let mut new_weight = 0;

        for (i, e_i) in e.iter().enumerate() {
            if i == dropped_i || tree.issame(e_i.0, e_i.1) {
                continue;
            }

            tree.unite(e_i.0, e_i.1);
            new_weight += e_i.2;
        }

        if new_weight > weight || tree.n != 1 {
            ret.push(e.get(dropped_i).unwrap().clone());
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            q_15_3(4, vec![(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 2),]),
            vec![(2, 3, 2)],
        )
    }
}
