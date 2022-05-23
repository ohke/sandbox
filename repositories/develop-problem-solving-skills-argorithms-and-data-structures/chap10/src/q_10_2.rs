fn solve(a: Vec<u64>) -> Vec<u64> {
    let mut heap = Vec::new();

    for a_i in a.iter().cloned() {
        heap.push(a_i);

        let mut i = heap.len() - 1;
        while i > 0 {
            let parent_i = (i + 1) / 2 - 1;
            if heap[i] > heap[parent_i] {
                heap.swap(i, parent_i);
                i = parent_i;
            } else {
                break;
            }
        }
    }

    heap
}

#[cfg(test)]
mod tests {
    use super::*;

    // q_10_2
    #[test]
    fn test_1() {
        assert_eq!(solve(vec![5, 6, 1]), vec![6, 5, 1]);
    }

    // q_10_3
    #[test]
    fn test_2() {
        // 65127 -> 65
        assert_eq!(solve(vec![5, 6, 1, 2, 7, 3, 4]), vec![7, 6, 4, 2, 5, 1, 3]);
    }
}
