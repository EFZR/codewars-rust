/*
Create a function that takes a Roman numeral as its argument and returns its value as a numeric decimal integer.
You don't need to validate the form of the Roman numeral.

Modern Roman numerals are written by expressing each decimal digit of the number to be encoded separately,
starting with the leftmost digit and skipping any 0s.
So 1990 is rendered "MCMXC" (1000 = M, 900 = CM, 90 = XC) and 2008 is rendered "MMVIII" (2000 = MM, 8 = VIII).
The Roman numeral for 1666, "MDCLXVI", uses each letter in descending order.
*/

use std::collections::HashMap;

// My Solution ->

pub fn roman_as_num(roman: &str) -> u64 {
    let roman: Vec<char> = roman.chars().collect();
    let mut result: Vec<u64> = Vec::new();
    for i in roman {
        match i {
            'I' => result.push(1),
            'V' => {
                if let Some(last) = result.last_mut() {
                    if *last < 5 {
                        *last = 5 - *last;
                    } else {
                        result.push(5)
                    }
                } else {
                    result.push(5)
                }
            }
            'X' => {
                if let Some(last) = result.last_mut() {
                    if *last < 10 {
                        *last = 10 - *last;
                    } else {
                        result.push(10)
                    }
                } else {
                    result.push(10);
                }
            }
            'L' => {
                if let Some(last) = result.last_mut() {
                    if *last < 50 {
                        *last = 50 - *last;
                    } else {
                        result.push(50)
                    }
                } else {
                    result.push(50);
                }
            }
            'C' => {
                if let Some(last) = result.last_mut() {
                    if *last < 100 {
                        *last = 100 - *last;
                    } else {
                        result.push(100)
                    }
                } else {
                    result.push(100);
                }
            }
            'D' => {
                if let Some(last) = result.last_mut() {
                    if *last < 500 {
                        *last = 500 - *last;
                    } else {
                        result.push(500)
                    }
                } else {
                    result.push(500);
                }
            }
            'M' => {
                if let Some(last) = result.last_mut() {
                    if *last < 1000 {
                        *last = 1000 - *last;
                    } else {
                        result.push(1000)
                    }
                } else {
                    result.push(1000);
                }
            }
            _ => (),
        }
    }
    result.iter().sum()
}

// ChatGPT Solution ->

pub fn _roman_as_num(roman: &str) -> u64 {
    let roman_numerals = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    let roman: Vec<char> = roman.chars().collect();
    let mut result: Vec<u64> = Vec::new();

    for &i in &roman {
        if let Some(&num) = roman_numerals.get(&i) {
            if let Some(last) = result.last_mut() {
                if *last < num {
                    *last = num - *last;
                } else {
                    result.push(num);
                }
            } else {
                result.push(num);
            }
        }
    }

    result.iter().sum()
}

// Clever Solution ->

fn __roman_as_num(roman: &str) -> u64 {
    let mut sum = 0;
    let mut prev = 0;

    let sym_val: HashMap<char, u64> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1_000),
    ]);

    for sym in roman.chars().rev() {
        let val = sym_val[&sym];
        if val >= prev {
            sum += val;
        } else {
            sum -= val;
        }
        prev = val;
    }

    sum
}

#[cfg(test)]
mod roman_as_num_tests {
    use super::roman_as_num;

    fn test_equal(input: &str, actual: u64, expected: u64) {
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) for the input \"{}\"",
            input
        );
    }

    #[test]
    fn basic() {
        test_equal("", roman_as_num(""), 0);
        test_equal("I", roman_as_num("I"), 1);
        test_equal("XXI", roman_as_num("XXI"), 21);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MDCLXVI", roman_as_num("MDCLXVI"), 1666);
    }

    #[test]
    fn one_through_ten() {
        test_equal("I", roman_as_num("I"), 1);
        test_equal("II", roman_as_num("II"), 2);
        test_equal("III", roman_as_num("III"), 3);
        test_equal("IV", roman_as_num("IV"), 4);
        test_equal("V", roman_as_num("V"), 5);
        test_equal("VI", roman_as_num("VI"), 6);
        test_equal("VII", roman_as_num("VII"), 7);
        test_equal("VIII", roman_as_num("VIII"), 8);
        test_equal("IX", roman_as_num("IX"), 9);
        test_equal("X", roman_as_num("X"), 10);
    }

    #[test]
    fn big_numbers() {
        test_equal("C", roman_as_num("C"), 100);
        test_equal("CDXLIV", roman_as_num("CDXLIV"), 444);
        test_equal("M", roman_as_num("M"), 1000);
        test_equal("MCMLIV", roman_as_num("MCMLIV"), 1954);
        test_equal("MCMXC", roman_as_num("MCMXC"), 1990);
        test_equal("MM", roman_as_num("MM"), 2000);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MMM", roman_as_num("MMM"), 3000);
        test_equal("MMMCM", roman_as_num("MMMCM"), 3900);
        test_equal("MMMCMXIV", roman_as_num("MMMCMXIV"), 3914);
    }
}
