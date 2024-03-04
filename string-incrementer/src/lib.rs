/*
Your job is to write a function which increments a string, to create a new string.

If the string already ends with a number, the number should be incremented by 1.
If the string does not end with a number. the number 1 should be appended to the new string.

Attention: If the number has leading zeros the amount of digits should be considered.
*/

pub fn increment_string(s: &str) -> String {
    if let Some(last) = s.chars().last() {
        match last.to_digit(10) {
            Some(9) => format!("{}0", &increment_string(&s[..s.len()-1])),
            Some(num) => format!("{}{}",&s[..s.len()-1],num+1),
            None => format!("{s}1")
        }
    } else {format!("1")}
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_foo() {
                assert_eq!(increment_string("foo"), "foo1", "Failed on input 'foo'");
            }

            #[test]
            fn test_foobar001() {
                assert_eq!(
                    increment_string("foobar001"),
                    "foobar002",
                    "Failed on input 'foobar001'"
                );
            }

            #[test]
            fn test_foobar1() {
                assert_eq!(
                    increment_string("foobar1"),
                    "foobar2",
                    "Failed on input 'foobar1'"
                );
            }

            #[test]
            fn test_foobar00() {
                assert_eq!(
                    increment_string("foobar00"),
                    "foobar01",
                    "Failed on input 'foobar00'"
                );
            }

            #[test]
            fn test_foobar99() {
                assert_eq!(
                    increment_string("foobar99"),
                    "foobar100",
                    "Failed on input 'foobar99'"
                );
            }

            #[test]
            fn test_foobar099() {
                assert_eq!(
                    increment_string("foobar099"),
                    "foobar100",
                    "Failed on input 'foobar099'"
                );
            }

            #[test]
            fn test_empty_string() {
                assert_eq!(increment_string(""), "1", "Failed on input ''");
            }
        }
    }
}
