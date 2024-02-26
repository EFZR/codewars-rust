// my solution ->
pub fn digital_root(n: i64) -> i64 {
    if n > 9 {
        let sum_of_digits = n
            .to_string()
            .chars()
            .map(|f| f.to_digit(10).unwrap_or(0) as i64)
            .sum();

        return digital_root(sum_of_digits);
    }

    n
}

// clever solution ->
fn _digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(123), 6);
    }

    #[test]
    fn returns_expected_for_single_digit() {
        assert_eq!(digital_root(7), 7);
    }

    #[test]
    fn returns_expected_for_two_digits() {
        assert_eq!(digital_root(45), 9);
    }

    #[test]
    fn returns_expected_for_large_number() {
        assert_eq!(digital_root(9876543210), 9);
    }

    #[test]
    fn returns_expected_for_zero() {
        assert_eq!(digital_root(0), 0);
    }
}
