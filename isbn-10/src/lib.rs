/*
ISBN-10 identifiers are ten digits long. The first nine characters are digits 0-9. The last digit can be 0-9 or X, to indicate a value of 10.

An ISBN-10 number is valid if the sum of the digits multiplied by their position modulo 11 equals zero.
*/

pub fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10
        || isbn[0..8].chars().any(|f| !f.is_digit(10))
        || isbn.chars().last().unwrap() != 'X' && !isbn.chars().last().unwrap().is_digit(10)
    {
        return false;
    }

    let isbn: Vec<i32> = isbn
        .chars()
        .map(|f| f.to_digit(10).or(Some(10)).unwrap() as i32)
        .collect();

    isbn.iter()
        .enumerate()
        .map(|(i, &num)| num * (i as i32 + 1))
        .sum::<i32>()
        % 11
        == 0
}

pub fn _valid_isbn10(isbn: &str) -> bool {
    isbn.len() == 10 &&
    isbn.chars().enumerate().all(|(i, c)| c.is_numeric() || (c == 'X' && i == 9)) &&
    isbn.chars().enumerate().map(|(i, c)| c.to_digit(10).unwrap_or(10) * (i as u32 + 1)).sum::<u32>() % 11 == 0
}

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(
            actual == expected,
            "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
