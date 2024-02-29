// my solution ->
pub fn perimeter(n: u64) -> u64 {
    let mut square: Vec<u64> = vec![0, 1];
    square.resize(n as usize + 2, 0);

    for i in 2..=n + 1 {
        square[i as usize] = square[i as usize - 1] + square[i as usize - 2];
    }

    square.iter().sum::<u64>() * 4
}

//clever solution ->
pub fn _perimeter(n: u64) -> u64 {
    (0..n)
        .fold((1u64, 1u64, 1u64), |a, _| (a.1, a.0 + a.1, a.1 + a.2))
        .2
        * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }

    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}
