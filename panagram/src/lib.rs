/*
A pangram is a sentence that contains every single letter of the alphabet at least once.
For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram,
because it uses the letters A-Z at least once (case is irrelevant).

Given a string, detect whether or not it is a pangram.
Return True if it is, False if not. Ignore numbers and punctuation.
*/

// My solution ->
pub fn is_pangram(s: &str) -> bool {
    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    ASCII_LOWER
        .iter()
        .all(|&l| s.to_lowercase().chars().any(|c| l == c))
}

// Clever solution ->
use std::collections::HashSet;

fn _is_pangram(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

// My solution improved
pub fn __is_pangram(s: &str) -> bool {
    ('a'..'z').all(|l| s.to_lowercase().chars().any(|c| l == c))
}

#[cfg(test)]
mod tests {
    use super::__is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = __is_pangram(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
