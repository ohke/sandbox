fn solve(s: String) -> Result<Vec<(u64, u64)>, ()> {
    let mut ret = Vec::new();
    let mut stack = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            if let Some(j) = stack.pop() {
                ret.push((j as u64, i as u64));
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
    }

    if stack.len() > 0 {
        return Err(());
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve("(()())".to_string()),
            Ok(vec![(1, 2), (3, 4), (0, 5)])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("(()()))".to_string()), Err(()));
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("((()())".to_string()), Err(()));
    }
}
