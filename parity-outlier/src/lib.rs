/*
You are given an array (which will have a length of at least 3, but could be very large) containing integers.
The array is either entirely comprised of odd integers or entirely comprised of even integers except for a single integer N.
Write a method that takes the array as an argument and returns this "outlier" N.
*/

pub fn find_outlier(values: &[i32]) -> i32 {
    let mut odd: Vec<i32> = Vec::new();
    let mut even: Vec<i32> = Vec::new();

    for &value in values {
        if value % 2 == 0 {
            even.push(value);
        } else {
            odd.push(value);
        }
    }

    if even.len() == 1 {
        even[0]
    } else {
        odd[0]
    }
}

// Better solution ->
pub fn find_outlier_other_solution(values: &[i32]) -> i32 {
    let sum: i32 = values.iter()
        .take(3)
        .map(|n| n.abs() % 2)
        .sum();

    let m = if sum == 1 || sum == 0 { 1 } else { 0 };

    values
        .iter()
        .find(|n| n.abs() % 2 == m)
        .map(|n| *n)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
