/*
You live in the city of Cartesia where all roads are laid out in a perfect grid.
You arrived ten minutes too early to an appointment, so you decided to take the opportunity to go for a short walk.
The city provides its citizens with a Walk Generating App on their phones -- everytime you press the button it
sends you an array of one-letter strings representing directions to walk (eg. ['n', 's', 'w', 'e']).
You always walk only a single block for each letter (direction) and you know it takes you one minute to traverse one city block,
so create a function that will return true if the walk the app gives you will
take you exactly ten minutes (you don't want to be early or late!) and will, of course, return you to your starting point.
Return false otherwise.
*/

pub fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }

    let mut vertical: i8 = 0;
    let mut horizontal: i8 = 0;

    for &direction in walk {
        match direction {
            'n' => vertical += 1,
            's' => vertical -= 1,
            'e' => horizontal += 1,
            'w' => horizontal -= 1,
            _ => ()
        }
    }

    vertical == 0 && horizontal == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
