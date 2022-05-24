fn solve(n: usize, e: Vec<(u64, u64)>) -> u64 {
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

    let mut ret = 0;

    for i in 0..e.len() {
        let mut ut = UnionTree::new(n);

        for j in 0..e.len() {
            if i == j {
                continue;
            }

            ut.unite(e[j].0, e[j].1);
        }

        if ut.num_roots() > 1 {
            ret += 1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(3, vec![(0, 1), (0, 2), (1, 2)]), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(3, vec![(0, 1), (0, 2)]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(3, vec![(0, 1), (0, 2)]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve(4, vec![(0, 1), (1, 2), (2, 3)]), 3);
    }
}
