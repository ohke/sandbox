fn solve(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 0u64;
    } else if n == 2 {
        return 1u64;
    }

    solve(n - 1) + solve(n - 2) + solve(n - 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(8), 24);
    }
}
