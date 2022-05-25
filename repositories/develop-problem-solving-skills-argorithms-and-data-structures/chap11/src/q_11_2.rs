fn solve(n: usize, e: Vec<(u64, u64)>) -> Vec<u64> {
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

    let mut ret = Vec::with_capacity(e.len());

    let mut ut = UnionTree::new(n);
    ret.push(ut.num_roots());

    for i in (0..e.len()).rev() {
        ut.unite(e[i].0, e[i].1);
        ret.insert(0, ut.num_roots());
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(3, vec![(0, 1), (0, 2), (1, 2)]), vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
            vec![1, 2, 3, 4, 5]
        );
    }
}
