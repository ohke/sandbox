fn solve(n: usize, k: Vec<(u64, u64)>, l: Vec<(u64, u64)>) -> Vec<u64> {
    struct UnionTree {
        parents: Vec<Option<u64>>,
        sizes: Vec<u64>,
    }

    impl UnionTree {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![None; n],
                sizes: vec![1; n],
            }
        }

        fn root(&mut self, a: u64) -> u64 {
            match self.parents[a as usize] {
                Some(p) => {
                    let r = self.root(p);
                    self.parents[a as usize] = Some(r);
                    r
                }
                None => a,
            }
        }

        pub fn issame(&mut self, a: u64, b: u64) -> bool {
            self.root(a) == self.root(b)
        }

        pub fn num_roots(&self) -> u64 {
            self.parents.iter().fold(0, |sum, elem| {
                sum + match elem {
                    Some(_) => 0,
                    None => 1,
                }
            })
        }

        pub fn unite(&mut self, a: u64, b: u64) {
            let r_a = self.root(a);
            let r_b = self.root(b);
            if r_a == r_b {
                return;
            }

            let s_a = self.sizes[r_a as usize];
            let s_b = self.sizes[r_b as usize];
            if s_a > s_b {
                self.parents[r_b as usize] = Some(r_a);
                self.sizes[r_a as usize] += s_b;
                self.sizes[r_b as usize] = 0;
            } else {
                self.parents[r_a as usize] = Some(r_b);
                self.sizes[r_a as usize] = 0;
                self.sizes[r_b as usize] = s_a;
            }
        }
    }

    let mut k_ut = UnionTree::new(n);
    for k_i in k.iter() {
        k_ut.unite(k_i.0, k_i.1);
    }

    let mut l_ut = UnionTree::new(n);
    for l_i in l.iter() {
        l_ut.unite(l_i.0, l_i.1);
    }

    let mut ret = Vec::new();
    for i in (0..n).map(|v| v as u64) {
        let mut count = 0;
        for j in (0..n).map(|v| v as u64) {
            if k_ut.issame(i, j) && l_ut.issame(i, j) {
                count += 1;
            }
        }

        ret.push(count);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(4, vec![(0, 1), (0, 2)], vec![(0, 3), (1, 3)]),
            vec![2, 2, 1, 1]
        );
    }
}
