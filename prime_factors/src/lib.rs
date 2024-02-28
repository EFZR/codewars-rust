/*
Given a positive number n > 1 find the prime factor decomposition of n. The result will be a string with the following form : "(p1n1)(p2n2)...(pk**nk)"
*/

// Honestly not my solution ->
pub fn prime_factors(n: i64) -> String {
    let mut result = String::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
        } else {
            n /= i;
            let mut count = 1;
            while n % i == 0 {
                n /= i;
                count += 1;
            }
            if count > 1 {
                result.push_str(&format!("({}**{})", i, count));
            } else {
                result.push_str(&format!("({})", i));
            }
        }
    }

    if n > 1 {
        result.push_str(&format!("({})", n));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
