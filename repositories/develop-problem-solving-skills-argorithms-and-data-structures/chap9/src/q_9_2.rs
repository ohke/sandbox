fn solve(s: String) -> i64 {
    let mut stack = Vec::new();

    for elem in s.split_ascii_whitespace() {
        if let Ok(num) = elem.parse::<i64>() {
            stack.push(num);
            continue;
        }

        let op1 = stack.pop().unwrap();
        let op2 = stack.pop().unwrap();

        let v = match elem {
            "+" => op2 + op1,
            "-" => op2 - op1,
            "*" => op2 * op1,
            "/" => op2 / op1,
            _ => panic!(),
        };

        stack.push(v);
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("3 4 + 1 2 - * 7 /".to_string()), -1);
    }
}
