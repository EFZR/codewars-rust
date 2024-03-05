// Given an array of positive or negative integers
// I= [i1,..,in]
// you have to produce a sorted array P of the form
// [ [p, sum of all ij of I for which p is a prime factor (p positive) of ij] ...]
// P will be sorted by increasing order of the prime numbers.

use std::collections::BTreeSet;

pub fn is_prime(n: i64) -> bool {
    let upper_bound = (n as f64).sqrt().floor() as i64 + 1;
    (2..upper_bound).all(|i| n % i !=0)
}

pub fn get_all_divisors(l: &Vec<i64>) -> BTreeSet<i64> {
    let upper_bound = l.iter().map(|x| x.abs()).max().unwrap_or(0) + 1;
    (2..upper_bound).filter(|x| is_prime(*x)).filter(|x| l.iter().any(|y| *y % x == 0)).collect()
}

pub fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let divisors = get_all_divisors(&l);
    divisors.iter().map(|x| (*x, l.iter().filter(|y| **y % *x == 0).sum())).collect()
}
#[cfg(test)]
mod test {
    use super::*;

    fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(sum_of_divided(l), exp)
    }

    #[test]
    fn basics_sum_of_divided() {
        testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
        testing(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
    }
}
