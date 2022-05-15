fn solve(a: u64, b: u64, c: u64) -> f64 {
    let mut ret = 0.0;

    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let mut left = 0.0;
    let mut right = 200.0;
    let mut nearest: f64 = 0.0;
    for _ in 0..64 {
        let t_i = left + (right - left) / 2.0;

        let v = a * t_i + b * (c * t_i * std::f64::consts::PI).sin();
        if (v - 100.0).abs() < (nearest - 100.0).abs() {
            nearest = v;
            ret = t_i;
        }

        if v < 100.0 {
            left = t_i;
        } else {
            right = t_i;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = solve(1, 1, 1);
        println!("{}", ret);
        assert!((100.0 - 0.0000001) < ret && ret < (100.0 + 0.0000001));
    }

    #[test]
    fn test_2() {
        let ret = solve(53, 82, 49);
        let v = 53.0 * ret + 82.0 * (49.0 * ret * std::f64::consts::PI).sin();
        assert!((100.0 - 0.00000001) < v && v < (100.0 + 0.0000001));
    }
}
